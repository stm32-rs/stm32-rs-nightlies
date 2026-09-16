///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `OUT2_RMP` reader - OUT2_RMP
pub type OUT2_RMP_R = crate::BitReader;
///Field `OUT2_RMP` writer - OUT2_RMP
pub type OUT2_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSCOEN` reader - LSCOEN
pub type LSCOEN_R = crate::FieldReader;
///Field `LSCOEN` writer - LSCOEN
pub type LSCOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - OUT2_RMP
    #[inline(always)]
    pub fn out2_rmp(&self) -> OUT2_RMP_R {
        OUT2_RMP_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - LSCOEN
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("out2_rmp", &self.out2_rmp())
            .field("lscoen", &self.lscoen())
            .finish()
    }
}
impl W {
    ///Bit 0 - OUT2_RMP
    #[inline(always)]
    pub fn out2_rmp(&mut self) -> OUT2_RMP_W<'_, CFGRrs> {
        OUT2_RMP_W::new(self, 0)
    }
    ///Bits 1:2 - LSCOEN
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W<'_, CFGRrs> {
        LSCOEN_W::new(self, 1)
    }
}
/**RTC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RTC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
