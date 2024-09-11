import 'package:flutter/material.dart';
import 'globals.dart' as globals;

class MapWidget extends StatefulWidget {
  @override
  _MapWidgetState createState() => _MapWidgetState();
}

class _MapWidgetState extends State<MapWidget> {
  double _currentScale = 1.0;
  final double _initialMarkerSize = 15.0;
  final double _imageWidth = 2238;
  final double _imageHeight = 2048;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        double displayWidth = constraints.maxWidth;
        double displayHeight = constraints.maxHeight;
        print('Display width: $displayWidth, Display height: $displayHeight');
        double scaleX = displayWidth / _imageWidth;
        double scaleY = displayHeight / _imageHeight;
        print('Scale X: $scaleX, Scale Y: $scaleY');
        print('marker x: ${1975 * scaleX}, marker y: ${1809 * scaleY}');
        double markerSize = 5;
        double markerLeft = (globals.latitude * scaleX) - (markerSize / 2);
        double markerTop = (globals.longitude * scaleY) - markerSize;

        return InteractiveViewer(
          minScale: 0.1,
          maxScale: 10.0,
          onInteractionUpdate: (ScaleUpdateDetails details) {
            setState(() {
              _currentScale = details.scale;
            });
          },
          child: Stack(
            children: [
              Image.network(
                'https://upload.wikimedia.org/wikipedia/commons/thumb/4/48/Hong_Kong_Railway_Route_Map_en.svg/2238px-Hong_Kong_Railway_Route_Map_en.svg.png',
                width: _imageWidth,
                height: _imageHeight,
                fit: BoxFit.fitWidth,
              ),
              Positioned(
                left: markerLeft, // X coordinate of the marker
                top: markerTop, // Y coordinate of the marker
                child: Icon(
                  Icons.location_on,
                  color: Colors.red,
                  size: markerSize,
                ),
              ),
              // Add more Positioned widgets for additional markers
            ],
          ),
        );
      },
    );
  }
}
