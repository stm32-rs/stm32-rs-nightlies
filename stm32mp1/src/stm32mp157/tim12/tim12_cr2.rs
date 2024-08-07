///Register `TIM12_CR2` reader
pub type R = crate::R<TIM12_CR2rs>;
///Register `TIM12_CR2` writer
pub type W = crate::W<TIM12_CR2rs>;
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
        f.debug_struct("TIM12_CR2")
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - MMS
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<TIM12_CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1S
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<TIM12_CR2rs> {
        TI1S_W::new(self, 7)
    }
}
/**TIM12 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim12_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM12:TIM12_CR2)*/
pub struct TIM12_CR2rs;
impl crate::RegisterSpec for TIM12_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`tim12_cr2::R`](R) reader structure
impl crate::Readable for TIM12_CR2rs {}
///`write(|w| ..)` method takes [`tim12_cr2::W`](W) writer structure
impl crate::Writable for TIM12_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM12_CR2 to value 0
impl crate::Resettable for TIM12_CR2rs {
    const RESET_VALUE: u32 = 0;
}
