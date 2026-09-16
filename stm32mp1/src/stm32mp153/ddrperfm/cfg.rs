///Register `CFG` reader
pub type R = crate::R<CFGrs>;
///Register `CFG` writer
pub type W = crate::W<CFGrs>;
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
        f.debug_struct("CFG")
            .field("en", &self.en())
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CFGrs> {
        EN_W::new(self, 0)
    }
    ///Bits 16:17 - SEL
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<'_, CFGrs> {
        SEL_W::new(self, 16)
    }
}
/**DDRPERFM configurationl register

You can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:CFG)*/
pub struct CFGrs;
impl crate::RegisterSpec for CFGrs {
    type Ux = u32;
}
///`read()` method returns [`cfg::R`](R) reader structure
impl crate::Readable for CFGrs {}
///`write(|w| ..)` method takes [`cfg::W`](W) writer structure
impl crate::Writable for CFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFG to value 0
impl crate::Resettable for CFGrs {}
