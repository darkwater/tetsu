import 'package:ferry/ferry.dart' as ferry;
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:provider/provider.dart';
import 'package:tetsu/cubit/remote_cubit.dart';
import 'package:tetsu/pages/home/home.dart';

import 'consts.dart';
import 'preferences.dart';
import 'remote.dart';
import 'remote_manager.dart';

void main() async {
  await Hive.initFlutter();
  Hive.registerAdapter(RemoteAdapter());

  await Preferences.init();

  await Hive.openBox<Remote>(BoxNames.remotes);

  final remoteManager = RemoteManager();

  final remotes = remoteManager.remotes.values;
  final selectedRemote = remotes.isNotEmpty ? remotes.first : null;

  final remoteCubit = RemoteCubit();

  ferry.Client gqlClient = ferry.Client(
    link: Link.from([
      DedupeLink(),
      Link.function((request, [forward]) async* {
        Remote? selectedRemote;
        while (true) {
          selectedRemote = remoteCubit.state.remote;
          if (selectedRemote != null) break;

          await Future.delayed(const Duration(milliseconds: 200));
        }

        final link = await selectedRemote.gqlLink;
        yield* link.request(request, forward);
      }),
    ]),
  );

  if (selectedRemote != null) {
    remoteCubit.select(selectedRemote);
  }

  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider.value(value: remoteManager),
        BlocProvider.value(value: remoteCubit),
        Provider.value(value: gqlClient),
      ],
      child: const MyApp(),
    ),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: "Tetsu",
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      darkTheme: ThemeData(
        brightness: Brightness.dark,
        primarySwatch: Colors.blue,
      ),
      home: const HomePage(),
    );
  }
}
