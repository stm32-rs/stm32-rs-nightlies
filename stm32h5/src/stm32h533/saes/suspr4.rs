///Register `SUSPR4` reader
pub type R = crate::R<SUSPR4rs>;
///Register `SUSPR4` writer
pub type W = crate::W<SUSPR4rs>;
///Field `SUSP` reader - Suspend data
pub type SUSP_R = crate::FieldReader<u32>;
///Field `SUSP` writer - Suspend data
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Suspend data
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSPR4")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Suspend data
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, SUSPR4rs> {
        SUSP_W::new(self, 0)
    }
}
/**SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#SAES:SUSPR4)*/
pub struct SUSPR4rs;
impl crate::RegisterSpec for SUSPR4rs {
    type Ux = u32;
}
///`read()` method returns [`suspr4::R`](R) reader structure
impl crate::Readable for SUSPR4rs {}
///`write(|w| ..)` method takes [`suspr4::W`](W) writer structure
impl crate::Writable for SUSPR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SUSPR4 to value 0
impl crate::Resettable for SUSPR4rs {}
