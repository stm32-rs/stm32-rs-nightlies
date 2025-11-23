///Register `RFSWHSECR` reader
pub type R = crate::R<RFSWHSECRrs>;
///Register `RFSWHSECR` writer
pub type W = crate::W<RFSWHSECRrs>;
///Field `GMC` reader - GMC\[6:5\]: High speed external XO current control reference 00: 10 uA 01: 20 uA 1x: 40 uA GMC\[4:0\]: High speed external XO current control multiplying factor IcoreHSE= GMC\[4:0\] * GMC\[6:5\] Example: GMC\[6:0\]=0x1111001 -> IcoreHSE=25*40uA / Default 3F: IcoreHSE= 10uA x 31 = 310uA Note: this value is set only by software.
pub type GMC_R = crate::FieldReader;
///Field `GMC` writer - GMC\[6:5\]: High speed external XO current control reference 00: 10 uA 01: 20 uA 1x: 40 uA GMC\[4:0\]: High speed external XO current control multiplying factor IcoreHSE= GMC\[4:0\] * GMC\[6:5\] Example: GMC\[6:0\]=0x1111001 -> IcoreHSE=25*40uA / Default 3F: IcoreHSE= 10uA x 31 = 310uA Note: this value is set only by software.
pub type GMC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SWXOTUNEEN` reader - RF-HSE capacitor bank tuning by SW enable Set by software
pub type SWXOTUNEEN_R = crate::BitReader;
///Field `SWXOTUNEEN` writer - RF-HSE capacitor bank tuning by SW enable Set by software
pub type SWXOTUNEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWXOTUNE` reader - RF-HSE capacitor bank tuning value by SW Set by software
pub type SWXOTUNE_R = crate::FieldReader;
///Field `SWXOTUNE` writer - RF-HSE capacitor bank tuning value by SW Set by software
pub type SWXOTUNE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `ISTARTUP` reader - RF-HSE Startup current Set by software Default value 2
pub type ISTARTUP_R = crate::FieldReader;
///Field `ISTARTUP` writer - RF-HSE Startup current Set by software Default value 2
pub type ISTARTUP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AMPLTHRESH` reader - RF-HSE Amplitude Control threshold Set by software Default value 0
pub type AMPLTHRESH_R = crate::FieldReader;
///Field `AMPLTHRESH` writer - RF-HSE Amplitude Control threshold Set by software Default value 0
pub type AMPLTHRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:6 - GMC\[6:5\]: High speed external XO current control reference 00: 10 uA 01: 20 uA 1x: 40 uA GMC\[4:0\]: High speed external XO current control multiplying factor IcoreHSE= GMC\[4:0\] * GMC\[6:5\] Example: GMC\[6:0\]=0x1111001 -> IcoreHSE=25*40uA / Default 3F: IcoreHSE= 10uA x 31 = 310uA Note: this value is set only by software.
    #[inline(always)]
    pub fn gmc(&self) -> GMC_R {
        GMC_R::new((self.bits & 0x7f) as u8)
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
    ///Bits 14:15 - RF-HSE Startup current Set by software Default value 2
    #[inline(always)]
    pub fn istartup(&self) -> ISTARTUP_R {
        ISTARTUP_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:18 - RF-HSE Amplitude Control threshold Set by software Default value 0
    #[inline(always)]
    pub fn amplthresh(&self) -> AMPLTHRESH_R {
        AMPLTHRESH_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFSWHSECR")
            .field("gmc", &self.gmc())
            .field("swxotuneen", &self.swxotuneen())
            .field("swxotune", &self.swxotune())
            .field("istartup", &self.istartup())
            .field("amplthresh", &self.amplthresh())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - GMC\[6:5\]: High speed external XO current control reference 00: 10 uA 01: 20 uA 1x: 40 uA GMC\[4:0\]: High speed external XO current control multiplying factor IcoreHSE= GMC\[4:0\] * GMC\[6:5\] Example: GMC\[6:0\]=0x1111001 -> IcoreHSE=25*40uA / Default 3F: IcoreHSE= 10uA x 31 = 310uA Note: this value is set only by software.
    #[inline(always)]
    pub fn gmc(&mut self) -> GMC_W<'_, RFSWHSECRrs> {
        GMC_W::new(self, 0)
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
    ///Bits 14:15 - RF-HSE Startup current Set by software Default value 2
    #[inline(always)]
    pub fn istartup(&mut self) -> ISTARTUP_W<'_, RFSWHSECRrs> {
        ISTARTUP_W::new(self, 14)
    }
    ///Bits 16:18 - RF-HSE Amplitude Control threshold Set by software Default value 0
    #[inline(always)]
    pub fn amplthresh(&mut self) -> AMPLTHRESH_W<'_, RFSWHSECRrs> {
        AMPLTHRESH_W::new(self, 16)
    }
}
/**RFSWHSECR register

You can [`read`](crate::Reg::read) this register and get [`rfswhsecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfswhsecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:RFSWHSECR)*/
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
///`reset()` method sets RFSWHSECR to value 0x803f
impl crate::Resettable for RFSWHSECRrs {
    const RESET_VALUE: u32 = 0x803f;
}
