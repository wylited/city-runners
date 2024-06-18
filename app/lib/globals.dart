library city_runners.globals;

String server_address = '';
String username = '';
String password = '';
String jwt = '';
bool initialized = false;

Future<void> initializeGlobals() async {
  final prefs = await SharedPreferences.getInstance();
  server_address = prefs.getString('server_address') ?? 'https://server.com';
  username = prefs.getString('username') ?? 'username';
  password = prefs.getString('password') ?? '';
  jwt = prefs.getString('jwt') ?? '1.1.1';
  initialized = true;
}
