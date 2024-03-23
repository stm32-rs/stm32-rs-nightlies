#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCRrs>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCRrs>;
#[doc = "Field `MODE1` reader - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN1=0."]
pub type MODE1_R = crate::FieldReader;
#[doc = "Field `MODE1` writer - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN1=0."]
pub type MODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODE2` reader - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2."]
pub type MODE2_R = crate::FieldReader;
#[doc = "Field `MODE2` writer - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2."]
pub type MODE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN1=0."]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2."]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample &amp; hold mode Note: This register can be modified only when EN1=0."]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<MCRrs> {
        MODE1_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2."]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<MCRrs> {
        MODE2_W::new(self, 16)
    }
}
#[doc = "DAC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCRrs {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCRrs {
    const RESET_VALUE: u32 = 0;
}
