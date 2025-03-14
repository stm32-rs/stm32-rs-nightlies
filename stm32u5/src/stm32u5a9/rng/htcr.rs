///Register `HTCR` reader
pub type R = crate::R<HTCRrs>;
///Register `HTCR` writer
pub type W = crate::W<HTCRrs>;
///Field `HTCFG` reader - health test configuration
pub type HTCFG_R = crate::FieldReader<u32>;
///Field `HTCFG` writer - health test configuration
pub type HTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - health test configuration
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HTCR")
            .field("htcfg", &self.htcfg())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - health test configuration
    #[inline(always)]
    pub fn htcfg(&mut self) -> HTCFG_W<HTCRrs> {
        HTCFG_W::new(self, 0)
    }
}
/**health test control register

You can [`read`](crate::Reg::read) this register and get [`htcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#RNG:HTCR)*/
pub struct HTCRrs;
impl crate::RegisterSpec for HTCRrs {
    type Ux = u32;
}
///`read()` method returns [`htcr::R`](R) reader structure
impl crate::Readable for HTCRrs {}
///`write(|w| ..)` method takes [`htcr::W`](W) writer structure
impl crate::Writable for HTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HTCR to value 0x6274
impl crate::Resettable for HTCRrs {
    const RESET_VALUE: u32 = 0x6274;
}
