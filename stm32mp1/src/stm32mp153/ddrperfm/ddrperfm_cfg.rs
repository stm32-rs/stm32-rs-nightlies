///Register `DDRPERFM_CFG` reader
pub type R = crate::R<DDRPERFM_CFGrs>;
///Register `DDRPERFM_CFG` writer
pub type W = crate::W<DDRPERFM_CFGrs>;
///Field `EN` reader - EN
pub type EN_R = crate::FieldReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEL` reader - SEL
pub type SEL_R = crate::FieldReader;
///Field `SEL` writer - SEL
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:17 - SEL
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRPERFM_CFG")
            .field("en", &self.en())
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - EN
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DDRPERFM_CFGrs> {
        EN_W::new(self, 0)
    }
    ///Bits 16:17 - SEL
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<DDRPERFM_CFGrs> {
        SEL_W::new(self, 16)
    }
}
/**DDRPERFM configurationl register

You can [`read`](crate::Reg::read) this register and get [`ddrperfm_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrperfm_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:DDRPERFM_CFG)*/
pub struct DDRPERFM_CFGrs;
impl crate::RegisterSpec for DDRPERFM_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`ddrperfm_cfg::R`](R) reader structure
impl crate::Readable for DDRPERFM_CFGrs {}
///`write(|w| ..)` method takes [`ddrperfm_cfg::W`](W) writer structure
impl crate::Writable for DDRPERFM_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRPERFM_CFG to value 0
impl crate::Resettable for DDRPERFM_CFGrs {
    const RESET_VALUE: u32 = 0;
}
