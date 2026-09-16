///Register `IDR` reader
pub type R = crate::R<IDRrs>;
/**Port C input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOC SECy.

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
///Field `ID13` reader - Port C input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOC SECy.
pub type ID13_R = crate::BitReader<INPUT_DATA>;
impl ID13_R {
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
///Field `ID14` reader - Port C input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOC SECy.
pub use ID13_R as ID14_R;
///Field `ID15` reader - Port C input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOC SECy.
pub use ID13_R as ID15_R;
impl R {
    ///Bit 13 - Port C input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port C input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port C input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR")
            .field("id13", &self.id13())
            .field("id14", &self.id14())
            .field("id15", &self.id15())
            .finish()
    }
}
/**GPIO port C input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GPIOC:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
