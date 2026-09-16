///Register `TSDR` reader
pub type R = crate::R<TSDRrs>;
///Field `DU` reader - Date units in BCD format
pub type DU_R = crate::FieldReader;
///Field `DT` reader - Date tens in BCD format
pub type DT_R = crate::FieldReader;
///Field `MU` reader - Month units in BCD format
pub type MU_R = crate::FieldReader;
///Field `MT` reader - Month tens in BCD format
pub type MT_R = crate::BitReader;
///Field `WDU` reader - Week day units
pub type WDU_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Week day units
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSDR")
            .field("wdu", &self.wdu())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .finish()
    }
}
/**time stamp date register

You can [`read`](crate::Reg::read) this register and get [`tsdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#RTC:TSDR)*/
pub struct TSDRrs;
impl crate::RegisterSpec for TSDRrs {
    type Ux = u32;
}
///`read()` method returns [`tsdr::R`](R) reader structure
impl crate::Readable for TSDRrs {}
///`reset()` method sets TSDR to value 0
impl crate::Resettable for TSDRrs {}
