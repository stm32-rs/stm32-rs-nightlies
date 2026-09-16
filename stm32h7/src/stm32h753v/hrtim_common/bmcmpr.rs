///Register `BMCMPR` reader
pub type R = crate::R<BMCMPRrs>;
///Register `BMCMPR` writer
pub type W = crate::W<BMCMPRrs>;
///Field `BMCMP` reader - BMCMP
pub type BMCMP_R = crate::FieldReader<u16>;
///Field `BMCMP` writer - BMCMP
pub type BMCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - BMCMP
    #[inline(always)]
    pub fn bmcmp(&self) -> BMCMP_R {
        BMCMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMCMPR")
            .field("bmcmp", &self.bmcmp())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - BMCMP
    #[inline(always)]
    pub fn bmcmp(&mut self) -> BMCMP_W<'_, BMCMPRrs> {
        BMCMP_W::new(self, 0)
    }
}
/**BMCMPR6

You can [`read`](crate::Reg::read) this register and get [`bmcmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HRTIM_Common:BMCMPR)*/
pub struct BMCMPRrs;
impl crate::RegisterSpec for BMCMPRrs {
    type Ux = u32;
}
///`read()` method returns [`bmcmpr::R`](R) reader structure
impl crate::Readable for BMCMPRrs {}
///`write(|w| ..)` method takes [`bmcmpr::W`](W) writer structure
impl crate::Writable for BMCMPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BMCMPR to value 0
impl crate::Resettable for BMCMPRrs {}
