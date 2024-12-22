///Register `CNT_remap` reader
pub type R = crate::R<CNT_REMAPrs>;
///Register `CNT_remap` writer
pub type W = crate::W<CNT_REMAPrs>;
///Field `CNT` reader - Least significant part of counter value
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - Least significant part of counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `UIFCPY` reader - UIF Copy
pub type UIFCPY_R = crate::BitReader;
///Field `UIFCPY` writer - UIF Copy
pub type UIFCPY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - Least significant part of counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - UIF Copy
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT_remap")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Least significant part of counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<CNT_REMAPrs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - UIF Copy
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W<CNT_REMAPrs> {
        UIFCPY_W::new(self, 31)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cnt_remap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt_remap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#TIM2:CNT_remap)*/
pub struct CNT_REMAPrs;
impl crate::RegisterSpec for CNT_REMAPrs {
    type Ux = u32;
}
///`read()` method returns [`cnt_remap::R`](R) reader structure
impl crate::Readable for CNT_REMAPrs {}
///`write(|w| ..)` method takes [`cnt_remap::W`](W) writer structure
impl crate::Writable for CNT_REMAPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNT_remap to value 0
impl crate::Resettable for CNT_REMAPrs {
    const RESET_VALUE: u32 = 0;
}
