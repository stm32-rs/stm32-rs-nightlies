///Register `IDR` reader
pub type R = crate::R<IDRrs>;
/**Port H input data I/O pin 3 This bit is read-only. It contain the input value of the corresponding I/O port. Access can be protected by GPIOH SEC3.

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
///Field `ID3` reader - Port H input data I/O pin 3 This bit is read-only. It contain the input value of the corresponding I/O port. Access can be protected by GPIOH SEC3.
pub type ID3_R = crate::BitReader<INPUT_DATA>;
impl ID3_R {
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
impl R {
    ///Bit 3 - Port H input data I/O pin 3 This bit is read-only. It contain the input value of the corresponding I/O port. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR").field("id3", &self.id3()).finish()
    }
}
/**GPIO port H input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GPIOH:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
