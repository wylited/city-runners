import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:geolocator/geolocator.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'dart:async';
import 'globals.dart' as globals;
import 'server.dart';
import 'login.dart';
import 'map.dart'; // Import the map.dart to use MapWidget

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await globals.initializeGlobals();
  runApp(const App());
}

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'CityRunners',
      themeMode: ThemeMode.system,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: globals.initialized
          ? const ServerSelectionPage()
          : const LoadingScreen(),
    );
  }
}

class LoadingScreen extends StatelessWidget {
  const LoadingScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: CircularProgressIndicator(),
      ),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  Position? _currentPosition;
  String _location = "Unknown";
  List<String> _logs = [];
  WebSocketChannel? _channel;
  Timer? _timer;

  @override
  void initState() {
    super.initState();
    _initializeWebSocket();
    _startLocationUpdates();
  }

  @override
  void dispose() {
    _timer?.cancel();
    _channel?.sink.close();
    super.dispose();
  }

  void _initializeWebSocket() {
    final String url =
        'wss://${globals.server_address}/ws?token=${globals.jwt}'; // Replace with your WebSocket server URL
    _channel = WebSocketChannel.connect(
      Uri.parse(url),
    );

    _channel?.stream.listen(
      (message) {
        setState(() {
          _logs.add('Received: $message');
        });
      },
      onError: (error) {
        setState(() {
          _logs.add('Error: $error');
        });
      },
      onDone: () {
        setState(() {
          _logs.add('WebSocket connection closed');
        });
      },
    );
  }

  void _startLocationUpdates() {
    _timer = Timer.periodic(const Duration(seconds: 30), (Timer timer) async {
      Position position = await Geolocator.getCurrentPosition(
          desiredAccuracy: LocationAccuracy.high);
      setState(() {
        _currentPosition = position;
        _location = 'Lat: ${position.latitude}, Lon: ${position.longitude}';
        _logs.add('Location updated: $_location');
      });
      final locationData = jsonEncode({
        'latitude': position.latitude,
        'longitude': position.longitude,
      });
      final uri = Uri.parse('https://${globals.server_address}/convert');
      final response = await http.post(
        uri,
        body: locationData,
        headers: {
          'Content-Type': 'application/json',
        },
      ); // returned value is gonna be json {"x": x, "y": y}
      final json = jsonDecode(response.body);
      globals.latitude = json['x'];
      globals.longitude = json['y'];
      setState(() {
        _logs.add('Converted: ${globals.latitude}, ${globals.longitude}');
      });

      if (_channel != null) {
        _channel?.sink.add(locationData);
        setState(() {
          _logs.add('Sent: $locationData');
        });
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Home Page'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              'Current Location: $_location',
              style: const TextStyle(fontSize: 16),
            ),
            const SizedBox(height: 16),
            const Text(
              'Logs:',
              style: TextStyle(fontSize: 16),
            ),
            Expanded(
              child: ListView.builder(
                itemCount: _logs.length,
                itemBuilder: (context, index) {
                  return Text(_logs[index]);
                },
              ),
            ),

            const SizedBox(height: 16),
            // Adding the MapWidget to the HomePage
            SizedBox(
              height: (MediaQuery.of(context).size.width - 32) * (2048 / 2238),
              child: MapWidget(),
            ),
          ],
        ),
      ),
    );
  }
}
