///Register `RFSWHSECR` reader
pub type R = crate::R<RFSWHSECRrs>;
///Register `RFSWHSECR` writer
pub type W = crate::W<RFSWHSECRrs>;
///Field `SATRG` reader - Sense Amplifier threshold Set by software.
pub type SATRG_R = crate::BitReader;
///Field `SATRG` writer - Sense Amplifier threshold Set by software.
pub type SATRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GMC` reader - High Speed External XO current control Set by software.
pub type GMC_R = crate::FieldReader;
///Field `GMC` writer - High Speed External XO current control Set by software.
pub type GMC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SWXOTUNEEN` reader - RF-HSE capacitor bank tuning by SW enable Set by software
pub type SWXOTUNEEN_R = crate::BitReader;
///Field `SWXOTUNEEN` writer - RF-HSE capacitor bank tuning by SW enable Set by software
pub type SWXOTUNEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWXOTUNE` reader - RF-HSE capacitor bank tuning value by SW Set by software
pub type SWXOTUNE_R = crate::FieldReader;
///Field `SWXOTUNE` writer - RF-HSE capacitor bank tuning value by SW Set by software
pub type SWXOTUNE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 3 - Sense Amplifier threshold Set by software.
    #[inline(always)]
    pub fn satrg(&self) -> SATRG_R {
        SATRG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - High Speed External XO current control Set by software.
    #[inline(always)]
    pub fn gmc(&self) -> GMC_R {
        GMC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - RF-HSE capacitor bank tuning by SW enable Set by software
    #[inline(always)]
    pub fn swxotuneen(&self) -> SWXOTUNEEN_R {
        SWXOTUNEEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - RF-HSE capacitor bank tuning value by SW Set by software
    #[inline(always)]
    pub fn swxotune(&self) -> SWXOTUNE_R {
        SWXOTUNE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFSWHSECR")
            .field("satrg", &self.satrg())
            .field("gmc", &self.gmc())
            .field("swxotuneen", &self.swxotuneen())
            .field("swxotune", &self.swxotune())
            .finish()
    }
}
impl W {
    ///Bit 3 - Sense Amplifier threshold Set by software.
    #[inline(always)]
    pub fn satrg(&mut self) -> SATRG_W<'_, RFSWHSECRrs> {
        SATRG_W::new(self, 3)
    }
    ///Bits 4:6 - High Speed External XO current control Set by software.
    #[inline(always)]
    pub fn gmc(&mut self) -> GMC_W<'_, RFSWHSECRrs> {
        GMC_W::new(self, 4)
    }
    ///Bit 7 - RF-HSE capacitor bank tuning by SW enable Set by software
    #[inline(always)]
    pub fn swxotuneen(&mut self) -> SWXOTUNEEN_W<'_, RFSWHSECRrs> {
        SWXOTUNEEN_W::new(self, 7)
    }
    ///Bits 8:13 - RF-HSE capacitor bank tuning value by SW Set by software
    #[inline(always)]
    pub fn swxotune(&mut self) -> SWXOTUNE_W<'_, RFSWHSECRrs> {
        SWXOTUNE_W::new(self, 8)
    }
}
/**RFSWHSECR register

You can [`read`](crate::Reg::read) this register and get [`rfswhsecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfswhsecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RCC:RFSWHSECR)*/
pub struct RFSWHSECRrs;
impl crate::RegisterSpec for RFSWHSECRrs {
    type Ux = u32;
}
///`read()` method returns [`rfswhsecr::R`](R) reader structure
impl crate::Readable for RFSWHSECRrs {}
///`write(|w| ..)` method takes [`rfswhsecr::W`](W) writer structure
impl crate::Writable for RFSWHSECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFSWHSECR to value 0x30
impl crate::Resettable for RFSWHSECRrs {
    const RESET_VALUE: u32 = 0x30;
}
