library city_runners.globals;

import 'package:shared_preferences/shared_preferences.dart';

String server_address = '';
String username = '';
String password = '';
String jwt = '';
bool initialized = false;
double latitude = 0.0;
double longitude = 0.0;

Future<void> initializeGlobals() async {
  final prefs = await SharedPreferences.getInstance();
  server_address = prefs.getString('server_address') ?? 'server.lol';
  username = prefs.getString('username') ?? 'username';
  password = prefs.getString('password') ?? '';
  jwt = prefs.getString('jwt') ?? '1.1.1';
  initialized = true;
  latitude = 0.0;
  longitude = 0.0;
}
