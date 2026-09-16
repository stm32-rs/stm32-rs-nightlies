///Register `CR0_LR` reader
pub type R = crate::R<CR0_LRrs>;
///Register `CR0_LR` writer
pub type W = crate::W<CR0_LRrs>;
///Field `CR_LR_GAIN_AFTER` reader - Set the gain of the clock recovery loop after Access Address detection to the value 2^(-CR_LR_GAIN_ AFTER) when the coded PHY is in use
pub type CR_LR_GAIN_AFTER_R = crate::FieldReader;
///Field `CR_LR_GAIN_AFTER` writer - Set the gain of the clock recovery loop after Access Address detection to the value 2^(-CR_LR_GAIN_ AFTER) when the coded PHY is in use
pub type CR_LR_GAIN_AFTER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CR_LR_GAIN_BEFORE` reader - Set the gain of the clock recovery loop before Access Address detection to the value 2^(-CR_LR_GAIN_ BEFORE) when the coded PHY is in use
pub type CR_LR_GAIN_BEFORE_R = crate::FieldReader;
///Field `CR_LR_GAIN_BEFORE` writer - Set the gain of the clock recovery loop before Access Address detection to the value 2^(-CR_LR_GAIN_ BEFORE) when the coded PHY is in use
pub type CR_LR_GAIN_BEFORE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Set the gain of the clock recovery loop after Access Address detection to the value 2^(-CR_LR_GAIN_ AFTER) when the coded PHY is in use
    #[inline(always)]
    pub fn cr_lr_gain_after(&self) -> CR_LR_GAIN_AFTER_R {
        CR_LR_GAIN_AFTER_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Set the gain of the clock recovery loop before Access Address detection to the value 2^(-CR_LR_GAIN_ BEFORE) when the coded PHY is in use
    #[inline(always)]
    pub fn cr_lr_gain_before(&self) -> CR_LR_GAIN_BEFORE_R {
        CR_LR_GAIN_BEFORE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR0_LR")
            .field("cr_lr_gain_after", &self.cr_lr_gain_after())
            .field("cr_lr_gain_before", &self.cr_lr_gain_before())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Set the gain of the clock recovery loop after Access Address detection to the value 2^(-CR_LR_GAIN_ AFTER) when the coded PHY is in use
    #[inline(always)]
    pub fn cr_lr_gain_after(&mut self) -> CR_LR_GAIN_AFTER_W<'_, CR0_LRrs> {
        CR_LR_GAIN_AFTER_W::new(self, 0)
    }
    ///Bits 4:7 - Set the gain of the clock recovery loop before Access Address detection to the value 2^(-CR_LR_GAIN_ BEFORE) when the coded PHY is in use
    #[inline(always)]
    pub fn cr_lr_gain_before(&mut self) -> CR_LR_GAIN_BEFORE_W<'_, CR0_LRrs> {
        CR_LR_GAIN_BEFORE_W::new(self, 4)
    }
}
/**CR0_LR register

You can [`read`](crate::Reg::read) this register and get [`cr0_lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0_lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:CR0_LR)*/
pub struct CR0_LRrs;
impl crate::RegisterSpec for CR0_LRrs {
    type Ux = u32;
}
///`read()` method returns [`cr0_lr::R`](R) reader structure
impl crate::Readable for CR0_LRrs {}
///`write(|w| ..)` method takes [`cr0_lr::W`](W) writer structure
impl crate::Writable for CR0_LRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR0_LR to value 0x66
impl crate::Resettable for CR0_LRrs {
    const RESET_VALUE: u32 = 0x66;
}
