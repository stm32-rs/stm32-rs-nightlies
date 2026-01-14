///Register `IAIER` reader
pub type R = crate::R<IAIERrs>;
///Register `IAIER` writer
pub type W = crate::W<IAIERrs>;
///Field `CAEIE` reader - Configuration access error interrupt enable
pub type CAEIE_R = crate::BitReader;
///Field `CAEIE` writer - Configuration access error interrupt enable
pub type CAEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAEIE` reader - Illegal access error interrupt enable
pub type IAEIE_R = crate::BitReader;
impl R {
    ///Bit 0 - Configuration access error interrupt enable
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Illegal access error interrupt enable
    #[inline(always)]
    pub fn iaeie(&self) -> IAEIE_R {
        IAEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IAIER")
            .field("caeie", &self.caeie())
            .field("iaeie", &self.iaeie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configuration access error interrupt enable
    #[inline(always)]
    pub fn caeie(&mut self) -> CAEIE_W<'_, IAIERrs> {
        CAEIE_W::new(self, 0)
    }
}
/**MCE illegal access interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`iaier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iaier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:IAIER)*/
pub struct IAIERrs;
impl crate::RegisterSpec for IAIERrs {
    type Ux = u32;
}
///`read()` method returns [`iaier::R`](R) reader structure
impl crate::Readable for IAIERrs {}
///`write(|w| ..)` method takes [`iaier::W`](W) writer structure
impl crate::Writable for IAIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IAIER to value 0
impl crate::Resettable for IAIERrs {}
