#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<SMPR2rs>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<SMPR2rs>;
#[doc = "Field `SMP10` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP10_R = crate::FieldReader;
#[doc = "Field `SMP10` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP10_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP11` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP11_R = crate::FieldReader;
#[doc = "Field `SMP11` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP11_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP12` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP12_R = crate::FieldReader;
#[doc = "Field `SMP12` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP12_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP13` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP13_R = crate::FieldReader;
#[doc = "Field `SMP13` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP13_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP14` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP14_R = crate::FieldReader;
#[doc = "Field `SMP14` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP14_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP15` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP15_R = crate::FieldReader;
#[doc = "Field `SMP15` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP15_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP16` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP16_R = crate::FieldReader;
#[doc = "Field `SMP16` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP16_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP17` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP17_R = crate::FieldReader;
#[doc = "Field `SMP17` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP17_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP18` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP18_R = crate::FieldReader;
#[doc = "Field `SMP18` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP18_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP19` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP19_R = crate::FieldReader;
#[doc = "Field `SMP19` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
pub type SMP19_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    pub fn smp19(&self) -> SMP19_R {
        SMP19_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<SMPR2rs> {
        SMP10_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<SMPR2rs> {
        SMP11_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<SMPR2rs> {
        SMP12_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<SMPR2rs> {
        SMP13_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<SMPR2rs> {
        SMP14_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<SMPR2rs> {
        SMP15_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<SMPR2rs> {
        SMP16_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<SMPR2rs> {
        SMP17_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp18(&mut self) -> SMP18_W<SMPR2rs> {
        SMP18_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\]
setting to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn smp19(&mut self) -> SMP19_W<SMPR2rs> {
        SMP19_W::new(self, 27)
    }
}
#[doc = "ADC sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR2rs;
impl crate::RegisterSpec for SMPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for SMPR2rs {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for SMPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2rs {
    const RESET_VALUE: u32 = 0;
}
