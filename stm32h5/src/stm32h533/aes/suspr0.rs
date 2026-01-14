///Register `SUSPR0` reader
pub type R = crate::R<SUSPR0rs>;
///Register `SUSPR0` writer
pub type W = crate::W<SUSPR0rs>;
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
        f.debug_struct("SUSPR0")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Suspend data
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, SUSPR0rs> {
        SUSP_W::new(self, 0)
    }
}
/**AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#AES:SUSPR0)*/
pub struct SUSPR0rs;
impl crate::RegisterSpec for SUSPR0rs {
    type Ux = u32;
}
///`read()` method returns [`suspr0::R`](R) reader structure
impl crate::Readable for SUSPR0rs {}
///`write(|w| ..)` method takes [`suspr0::W`](W) writer structure
impl crate::Writable for SUSPR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SUSPR0 to value 0
impl crate::Resettable for SUSPR0rs {}
