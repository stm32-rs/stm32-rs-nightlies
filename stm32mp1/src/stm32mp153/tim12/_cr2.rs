///Register `_CR2` reader
pub type R = crate::R<_CR2rs>;
///Register `_CR2` writer
pub type W = crate::W<_CR2rs>;
///Field `MMS` reader - MMS
pub type MMS_R = crate::FieldReader;
///Field `MMS` writer - MMS
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TI1S` reader - TI1S
pub type TI1S_R = crate::BitReader;
///Field `TI1S` writer - TI1S
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 4:6 - MMS
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1S
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_CR2")
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - MMS
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, _CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1S
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<'_, _CR2rs> {
        TI1S_W::new(self, 7)
    }
}
/**TIM12 control register 2

You can [`read`](crate::Reg::read) this register and get [`_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_CR2)*/
pub struct _CR2rs;
impl crate::RegisterSpec for _CR2rs {
    type Ux = u32;
}
///`read()` method returns [`_cr2::R`](R) reader structure
impl crate::Readable for _CR2rs {}
///`write(|w| ..)` method takes [`_cr2::W`](W) writer structure
impl crate::Writable for _CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _CR2 to value 0
impl crate::Resettable for _CR2rs {}
