Some early design thoughts - may not end up like this

API:
* serial scan
  * sequentially scan for devices on MBus bus using primary addresses
  * args
    * opt: baudrate
* serial request data
  * read data from given device. Supports both primary and secondary address types.
  * args
    * opt: baudrate
    * address
* serial switch baudrate
  * attempts to switch the communication speed of the MBus device
  * args
    * opt: baudrate
    * new baudrate
    * address
* serial scan secondary
  * scan for devices on MBus bus using secondary addresses. The scan is not sequential
  * args
    * opt: baudrate
    * address mask
* serial select seconday
  * perform single secondary address select to check what device responds.
  * args
    * opt: baudrate
    * secondary-mbus-address

Arg details:
* baudrate
  * 300, 600, 1200, 2400, 4800, 9600, 19200, 38400
* primary address
  * 1-250
* address mask for secondary scan
  * Address mask for secondary address scan - you can restrict the search by supplying an optional address mask on the form 'FFFFFFFFFFFFFFFF' where F is a wildcard character.
* primary or secondary address
  * MBus device address. Could be a primary address or a secondary address (sixteen digit hexadecimal number). The primary address should be an integer between 1 and 250 for addressing individual devices. However the program does allow using any one byte number (i.e. 0 to 255) so that you can also use addresses reserved for physical or data link layers management, secondary addressing, broadcasts, etc
* secondary address
  * MBus secondary address of the device. Sixteen digit hexadecimal number

use env variables for
* IP to bind to
* port to bind to
* device to use
* whether to be master or slave
* logging config

Think about
* whether to use retries
* whether to use nax response frames
* whether to use debug messages
