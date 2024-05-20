

class Wave:
    def __init__(self, wave_type, frequency, amplitude):
        self.wave_type = wave_type
        self.frequency = frequency
        self.amplitude = amplitude
        self.phase = 0

    def __str__(self):
        return f"{self.wave_type} wave at {self.frequency} Hz with amplitude {self.amplitude}"
    