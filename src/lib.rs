use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PhysicalPin(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct GpioPin(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct WiringPiPin(pub u8);

// Pin Translation Table
//
// | Physical | GPIO | WiringPi |
// |----------|------|----------|
// | 3        | 2    | 8        |
// | 5        | 3    | 9        |
// | 7        | 4    | 7        |
// | 8        | 14   | 15       |
// | 10       | 15   | 16       |
// | 11       | 17   | 0        |
// | 12       | 18   | 1        |
// | 13       | 27   | 2        |
// | 15       | 22   | 3        |
// | 16       | 23   | 4        |
// | 18       | 24   | 5        |
// | 19       | 10   | 12       |
// | 21       | 9    | 13       |
// | 22       | 25   | 6        |
// | 23       | 11   | 14       |
// | 24       | 8    | 10       |
// | 26       | 7    | 11       |
// | 27       | 0    | 30       |
// | 28       | 1    | 31       |
// | 29       | 5    | 21       |
// | 31       | 6    | 22       |
// | 32       | 12   | 26       |
// | 33       | 13   | 23       |
// | 35       | 19   | 24       |
// | 36       | 16   | 27       |
// | 37       | 26   | 25       |
// | 38       | 20   | 28       |
// | 40       | 21   | 29       |

impl From<PhysicalPin> for GpioPin {
    fn from(pin: PhysicalPin) -> Self {
        GpioPin(match pin.0 {
            3 => 2,
            5 => 3,
            7 => 4,
            8 => 14,
            10 => 15,
            11 => 17,
            12 => 18,
            13 => 27,
            15 => 22,
            16 => 23,
            18 => 24,
            19 => 10,
            21 => 9,
            22 => 25,
            23 => 11,
            24 => 8,
            26 => 7,
            27 => 0,
            28 => 1,
            29 => 5,
            31 => 6,
            32 => 12,
            33 => 13,
            35 => 19,
            36 => 16,
            37 => 26,
            38 => 20,
            40 => 21,
            _ => panic!("Invalid pin number"),
        })
    }
}

impl From<PhysicalPin> for WiringPiPin {
    fn from(pin: PhysicalPin) -> Self {
        WiringPiPin(match pin.0 {
            3 => 8,
            5 => 9,
            7 => 7,
            8 => 15,
            10 => 16,
            11 => 0,
            12 => 1,
            13 => 2,
            15 => 3,
            16 => 4,
            18 => 5,
            19 => 12,
            21 => 13,
            22 => 6,
            23 => 14,
            24 => 10,
            26 => 11,
            27 => 30,
            28 => 31,
            29 => 21,
            31 => 22,
            32 => 26,
            33 => 23,
            35 => 24,
            36 => 27,
            37 => 25,
            38 => 28,
            40 => 29,
            _ => panic!("Invalid pin number"),
        })
    }
}

impl From<GpioPin> for PhysicalPin {
    fn from(pin: GpioPin) -> Self {
        PhysicalPin(match pin.0 {
            2 => 3,
            3 => 5,
            4 => 7,
            14 => 8,
            15 => 10,
            17 => 11,
            18 => 12,
            27 => 13,
            22 => 15,
            23 => 16,
            24 => 18,
            10 => 19,
            9 => 21,
            25 => 22,
            11 => 23,
            8 => 24,
            7 => 26,
            0 => 27,
            1 => 28,
            5 => 29,
            6 => 31,
            12 => 32,
            13 => 33,
            19 => 35,
            16 => 36,
            26 => 37,
            20 => 38,
            21 => 40,
            _ => panic!("Invalid pin number"),
        })
    }
}

impl From<GpioPin> for WiringPiPin {
    fn from(pin: GpioPin) -> Self {
        WiringPiPin(match pin.0 {
            2 => 8,
            3 => 9,
            4 => 7,
            14 => 15,
            15 => 16,
            17 => 0,
            18 => 1,
            27 => 2,
            22 => 3,
            23 => 4,
            24 => 5,
            10 => 12,
            9 => 13,
            25 => 6,
            11 => 14,
            8 => 10,
            7 => 11,
            0 => 30,
            1 => 31,
            5 => 21,
            6 => 22,
            12 => 26,
            13 => 23,
            19 => 24,
            16 => 27,
            26 => 25,
            20 => 28,
            21 => 29,
            _ => panic!("Invalid pin number"),
        })
    }
}

impl From<WiringPiPin> for PhysicalPin {
    fn from(pin: WiringPiPin) -> Self {
        PhysicalPin(match pin.0 {
            8 => 3,
            9 => 5,
            7 => 7,
            15 => 8,
            16 => 10,
            0 => 11,
            1 => 12,
            2 => 13,
            3 => 15,
            4 => 16,
            5 => 18,
            12 => 19,
            13 => 21,
            6 => 22,
            14 => 23,
            10 => 24,
            11 => 26,
            30 => 27,
            31 => 28,
            21 => 29,
            22 => 31,
            26 => 32,
            23 => 33,
            24 => 35,
            27 => 36,
            25 => 37,
            28 => 38,
            29 => 40,
            _ => panic!("Invalid pin number"),
        })
    }
}

impl From<WiringPiPin> for GpioPin {
    fn from(pin: WiringPiPin) -> Self {
        GpioPin(match pin.0 {
            8 => 2,
            9 => 3,
            7 => 4,
            15 => 14,
            16 => 15,
            0 => 17,
            1 => 18,
            2 => 27,
            3 => 22,
            4 => 23,
            5 => 24,
            12 => 10,
            13 => 9,
            6 => 25,
            14 => 11,
            10 => 8,
            11 => 7,
            30 => 0,
            31 => 1,
            21 => 5,
            22 => 6,
            26 => 12,
            23 => 13,
            24 => 19,
            27 => 16,
            25 => 26,
            28 => 20,
            29 => 21,
            _ => panic!("Invalid pin number"),
        })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_physical_to_gpio() {
        assert_eq!(
            <WiringPiPin as Into<GpioPin>>::into(WiringPiPin(3)),
            GpioPin(22)
        );
        assert_eq!(
            <WiringPiPin as Into<GpioPin>>::into(WiringPiPin(5)),
            GpioPin(24)
        );
        assert_eq!(
            <WiringPiPin as Into<GpioPin>>::into(WiringPiPin(7)),
            GpioPin(4)
        );
        assert_eq!(
            <WiringPiPin as Into<GpioPin>>::into(WiringPiPin(8)),
            GpioPin(2)
        );
    }

    #[test]
    fn test_physical_to_wiringpi() {
        assert_eq!(
            <PhysicalPin as Into<WiringPiPin>>::into(PhysicalPin(3)),
            WiringPiPin(8)
        );
        assert_eq!(
            <PhysicalPin as Into<WiringPiPin>>::into(PhysicalPin(5)),
            WiringPiPin(9)
        );
        assert_eq!(
            <PhysicalPin as Into<WiringPiPin>>::into(PhysicalPin(7)),
            WiringPiPin(7)
        );
        assert_eq!(
            <PhysicalPin as Into<WiringPiPin>>::into(PhysicalPin(8)),
            WiringPiPin(15)
        );
    }

    #[test]
    fn test_gpio_to_physical() {
        assert_eq!(
            <GpioPin as Into<PhysicalPin>>::into(GpioPin(2)),
            PhysicalPin(3)
        );
        assert_eq!(
            <GpioPin as Into<PhysicalPin>>::into(GpioPin(3)),
            PhysicalPin(5)
        );
        assert_eq!(
            <GpioPin as Into<PhysicalPin>>::into(GpioPin(4)),
            PhysicalPin(7)
        );
        assert_eq!(
            <GpioPin as Into<PhysicalPin>>::into(GpioPin(14)),
            PhysicalPin(8)
        );
    }

    #[test]
    fn test_gpio_to_wiringpi() {
        assert_eq!(
            <GpioPin as Into<WiringPiPin>>::into(GpioPin(2)),
            WiringPiPin(8)
        );
        assert_eq!(
            <GpioPin as Into<WiringPiPin>>::into(GpioPin(3)),
            WiringPiPin(9)
        );
        assert_eq!(
            <GpioPin as Into<WiringPiPin>>::into(GpioPin(4)),
            WiringPiPin(7)
        );
        assert_eq!(
            <GpioPin as Into<WiringPiPin>>::into(GpioPin(14)),
            WiringPiPin(15)
        );
    }

    #[test]
    fn test_wiringpi_to_physical() {
        assert_eq!(
            <WiringPiPin as Into<PhysicalPin>>::into(WiringPiPin(8)),
            PhysicalPin(3)
        );
        assert_eq!(
            <WiringPiPin as Into<PhysicalPin>>::into(WiringPiPin(9)),
            PhysicalPin(5)
        );
        assert_eq!(
            <WiringPiPin as Into<PhysicalPin>>::into(WiringPiPin(7)),
            PhysicalPin(7)
        );
        assert_eq!(
            <WiringPiPin as Into<PhysicalPin>>::into(WiringPiPin(15)),
            PhysicalPin(8)
        );
    }

    #[test]
    fn test_wiringpi_to_gpio() {
        assert_eq!(
            <WiringPiPin as Into<GpioPin>>::into(WiringPiPin(8)),
            GpioPin(2)
        );
        assert_eq!(
            <WiringPiPin as Into<GpioPin>>::into(WiringPiPin(9)),
            GpioPin(3)
        );
        assert_eq!(
            <WiringPiPin as Into<GpioPin>>::into(WiringPiPin(7)),
            GpioPin(4)
        );
        assert_eq!(
            <WiringPiPin as Into<GpioPin>>::into(WiringPiPin(15)),
            GpioPin(14)
        );
    }
}
