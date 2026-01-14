///Register `MOD1_CONFIG` reader
pub type R = crate::R<MOD1_CONFIGrs>;
///Register `MOD1_CONFIG` writer
pub type W = crate::W<MOD1_CONFIGrs>;
///Field `FDEV_M` reader - Mantissa of the frequency deviation (default: 28.
pub type FDEV_M_R = crate::FieldReader;
///Field `FDEV_M` writer - Mantissa of the frequency deviation (default: 28.
pub type FDEV_M_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FDEV_E` reader - Exponent of the frequency deviation (default: 28.
pub type FDEV_E_R = crate::FieldReader;
///Field `FDEV_E` writer - Exponent of the frequency deviation (default: 28.
pub type FDEV_E_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CHFLT_M` reader - Mantissa of the channel filter BW (default: 100 kHz)
pub type CHFLT_M_R = crate::FieldReader;
///Field `CHFLT_M` writer - Mantissa of the channel filter BW (default: 100 kHz)
pub type CHFLT_M_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CHFLT_E` reader - Exponent of the channel filter BW (default: 100 kHz)
pub type CHFLT_E_R = crate::FieldReader;
///Field `CHFLT_E` writer - Exponent of the channel filter BW (default: 100 kHz)
pub type CHFLT_E_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Mantissa of the frequency deviation (default: 28.
    #[inline(always)]
    pub fn fdev_m(&self) -> FDEV_M_R {
        FDEV_M_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Exponent of the frequency deviation (default: 28.
    #[inline(always)]
    pub fn fdev_e(&self) -> FDEV_E_R {
        FDEV_E_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - Mantissa of the channel filter BW (default: 100 kHz)
    #[inline(always)]
    pub fn chflt_m(&self) -> CHFLT_M_R {
        CHFLT_M_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Exponent of the channel filter BW (default: 100 kHz)
    #[inline(always)]
    pub fn chflt_e(&self) -> CHFLT_E_R {
        CHFLT_E_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOD1_CONFIG")
            .field("fdev_m", &self.fdev_m())
            .field("fdev_e", &self.fdev_e())
            .field("chflt_m", &self.chflt_m())
            .field("chflt_e", &self.chflt_e())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Mantissa of the frequency deviation (default: 28.
    #[inline(always)]
    pub fn fdev_m(&mut self) -> FDEV_M_W<'_, MOD1_CONFIGrs> {
        FDEV_M_W::new(self, 0)
    }
    ///Bits 8:11 - Exponent of the frequency deviation (default: 28.
    #[inline(always)]
    pub fn fdev_e(&mut self) -> FDEV_E_W<'_, MOD1_CONFIGrs> {
        FDEV_E_W::new(self, 8)
    }
    ///Bits 16:19 - Mantissa of the channel filter BW (default: 100 kHz)
    #[inline(always)]
    pub fn chflt_m(&mut self) -> CHFLT_M_W<'_, MOD1_CONFIGrs> {
        CHFLT_M_W::new(self, 16)
    }
    ///Bits 20:23 - Exponent of the channel filter BW (default: 100 kHz)
    #[inline(always)]
    pub fn chflt_e(&mut self) -> CHFLT_E_W<'_, MOD1_CONFIGrs> {
        CHFLT_E_W::new(self, 20)
    }
}
/**MOD1_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`mod1_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod1_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:MOD1_CONFIG)*/
pub struct MOD1_CONFIGrs;
impl crate::RegisterSpec for MOD1_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`mod1_config::R`](R) reader structure
impl crate::Readable for MOD1_CONFIGrs {}
///`write(|w| ..)` method takes [`mod1_config::W`](W) writer structure
impl crate::Writable for MOD1_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOD1_CONFIG to value 0x0040_0435
impl crate::Resettable for MOD1_CONFIGrs {
    const RESET_VALUE: u32 = 0x0040_0435;
}
