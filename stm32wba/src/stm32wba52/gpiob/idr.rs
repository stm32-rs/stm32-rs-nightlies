///Register `IDR` reader
pub type R = crate::R<IDRrs>;
/**Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_DATA {
    ///0: Input is logic low
    Low = 0,
    ///1: Input is logic high
    High = 1,
}
impl From<INPUT_DATA> for bool {
    #[inline(always)]
    fn from(variant: INPUT_DATA) -> Self {
        variant as u8 != 0
    }
}
///Field `ID0` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub type ID0_R = crate::BitReader<INPUT_DATA>;
impl ID0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT_DATA {
        match self.bits {
            false => INPUT_DATA::Low,
            true => INPUT_DATA::High,
        }
    }
    ///Input is logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INPUT_DATA::Low
    }
    ///Input is logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INPUT_DATA::High
    }
}
///Field `ID1` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID1_R;
///Field `ID2` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID2_R;
///Field `ID3` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID3_R;
///Field `ID4` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID4_R;
///Field `ID5` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID5_R;
///Field `ID6` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID6_R;
///Field `ID7` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID7_R;
///Field `ID8` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID8_R;
///Field `ID9` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID9_R;
///Field `ID10` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID10_R;
///Field `ID11` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID11_R;
///Field `ID12` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID12_R;
///Field `ID13` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID13_R;
///Field `ID14` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID14_R;
///Field `ID15` reader - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use ID0_R as ID15_R;
impl R {
    ///Bit 0 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id8(&self) -> ID8_R {
        ID8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id9(&self) -> ID9_R {
        ID9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id10(&self) -> ID10_R {
        ID10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id11(&self) -> ID11_R {
        ID11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id12(&self) -> ID12_R {
        ID12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR")
            .field("id0", &self.id0())
            .field("id1", &self.id1())
            .field("id2", &self.id2())
            .field("id3", &self.id3())
            .field("id4", &self.id4())
            .field("id5", &self.id5())
            .field("id6", &self.id6())
            .field("id7", &self.id7())
            .field("id8", &self.id8())
            .field("id9", &self.id9())
            .field("id10", &self.id10())
            .field("id11", &self.id11())
            .field("id12", &self.id12())
            .field("id13", &self.id13())
            .field("id14", &self.id14())
            .field("id15", &self.id15())
            .finish()
    }
}
/**GPIO port B input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOB:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
