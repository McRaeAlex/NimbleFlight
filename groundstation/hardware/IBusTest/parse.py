# Run this file after running the logger, eventully we will combine them
from functools import partial
import json
import jsonpickle

class IBusPacket:
    """
    The format is 32 bytes long (2 header, 14 * 2 channels, 2 checksum)
    The data is little endian
    0x20 0x40 - Header
    0xXX 0xXX - Channel 1
    ...
    0xXX 0xXX - Channel 14
    0xYY 0xYY - Checksum calculated by substracting each channel from 0xFFFF
    """
    header = 0x2040
    channels = [1500] * 14 # Init all the channels to default values
    checksum = 0xFFFF # The checksum
    error = False # Set to True if the checksum doesn't match

    def __init__(self, buf: bytes):
        assert(len(buf) == 32)
        # assert(buf[0:2] == b'\x20\x40')

        self.header = int.from_bytes(buf[0:3], 'little')

        # parse the bytes into ints
        for i in range(14):
            start = 2 + i
            end = start + 2
            self.channels[0] = int.from_bytes(buf[start:end], 'little')

        self.checksum = int.from_bytes(buf[31:], 'little')

        self.checksum_verify()

    def checksum_verify(self):
        """Calculate the checksum and make sure it matches"""
        checksum_test = 0xFFFF

        # Sub each chan value from the start value to get the checksum
        for chan in self.channels:
            checksum_test = checksum_test - chan
        
        print(checksum_test, self.checksum)
        self.error = not (checksum_test == self.checksum)

if __name__ == '__main__':
    with open('./ibus_serial_out.hex', 'rb') as raw:
        with open('./ibus_serial_out.json', 'w+') as parsed:
            # First find the first magic number byte, we can assume that its our start point
            first_found = False
            for maybe_header in iter(partial(raw.read, 1), b''):
                if maybe_header == b'\x20':
                    print('Found first')
                    first_found = True
                    continue
                if first_found and maybe_header == b'\x40': 
                    print('Found')
                    break
                else:
                    first_found = False

            raw.seek(-2, 1) # Go back 2 bytes

            # consume 32 bytes at a time 
            packets = []

            for packet_raw in iter(partial(raw.read, 32), b''):
                if len(packet_raw) != 32:
                    break

                # TODO: This works 90% of the time... but not always it seems either the format becomes in order or we miss data
                packet = IBusPacket(packet_raw)
                packets.append(packet)

            # Write to the output file as json
            json.dump(jsonpickle.encode(packets), parsed)
