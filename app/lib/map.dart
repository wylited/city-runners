import 'package:flutter/material.dart';

class MapWidget extends StatefulWidget {
  @override
  _MapWidgetState createState() => _MapWidgetState();
}

class _MapWidgetState extends State<MapWidget> {
  double _currentScale = 1.0;
  final double _initialMarkerSize = 15.0;
  final double _imageWidth = 2238;
  final double _imageHeight = 2049;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        double displayWidth = constraints.maxWidth;
        double displayHeight = constraints.maxHeight;
        print('Display width: $displayWidth, Display height: $displayHeight');
        double scaleX = displayWidth / _imageWidth;
        double scaleY = displayHeight / _imageHeight;

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
                fit: BoxFit.cover,
              ),
              Positioned(
                left: 1975 * scaleX, // X coordinate of the marker
                top: 1809 * scaleY, // Y coordinate of the marker
                child: Icon(
                  Icons.location_on,
                  color: Colors.red,
                  size: 5,
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
