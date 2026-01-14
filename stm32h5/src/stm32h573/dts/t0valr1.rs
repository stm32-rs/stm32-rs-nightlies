///Register `T0VALR1` reader
pub type R = crate::R<T0VALR1rs>;
///Field `TS1_FMT0` reader - Engineering value of the frequency measured at T0 for temperature sensor 1 This value is expressed in 0.1 kHz.
pub type TS1_FMT0_R = crate::FieldReader<u16>;
///Field `TS1_T0` reader - Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used.
pub type TS1_T0_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Engineering value of the frequency measured at T0 for temperature sensor 1 This value is expressed in 0.1 kHz.
    #[inline(always)]
    pub fn ts1_fmt0(&self) -> TS1_FMT0_R {
        TS1_FMT0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:17 - Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used.
    #[inline(always)]
    pub fn ts1_t0(&self) -> TS1_T0_R {
        TS1_T0_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0VALR1")
            .field("ts1_fmt0", &self.ts1_fmt0())
            .field("ts1_t0", &self.ts1_t0())
            .finish()
    }
}
/**Temperature sensor T0 value register 1

You can [`read`](crate::Reg::read) this register and get [`t0valr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#DTS:T0VALR1)*/
pub struct T0VALR1rs;
impl crate::RegisterSpec for T0VALR1rs {
    type Ux = u32;
}
///`read()` method returns [`t0valr1::R`](R) reader structure
impl crate::Readable for T0VALR1rs {}
///`reset()` method sets T0VALR1 to value 0
impl crate::Resettable for T0VALR1rs {}
