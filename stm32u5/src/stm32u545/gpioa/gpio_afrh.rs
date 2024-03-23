#[doc = "Register `GPIO_AFRH` reader"]
pub type R = crate::R<GPIO_AFRHrs>;
#[doc = "Register `GPIO_AFRH` writer"]
pub type W = crate::W<GPIO_AFRHrs>;
#[doc = "Field `AFSEL8` reader - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL8_R = crate::FieldReader;
#[doc = "Field `AFSEL8` writer - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL9` reader - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL9_R = crate::FieldReader;
#[doc = "Field `AFSEL9` writer - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL10` reader - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL10_R = crate::FieldReader;
#[doc = "Field `AFSEL10` writer - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL11` reader - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL11_R = crate::FieldReader;
#[doc = "Field `AFSEL11` writer - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL12` reader - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL12_R = crate::FieldReader;
#[doc = "Field `AFSEL12` writer - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL13` reader - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL13_R = crate::FieldReader;
#[doc = "Field `AFSEL13` writer - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL14` reader - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL14_R = crate::FieldReader;
#[doc = "Field `AFSEL14` writer - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL15` reader - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL15_R = crate::FieldReader;
#[doc = "Field `AFSEL15` writer - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn afsel8(&mut self) -> AFSEL8_W<GPIO_AFRHrs> {
        AFSEL8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn afsel9(&mut self) -> AFSEL9_W<GPIO_AFRHrs> {
        AFSEL9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn afsel10(&mut self) -> AFSEL10_W<GPIO_AFRHrs> {
        AFSEL10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn afsel11(&mut self) -> AFSEL11_W<GPIO_AFRHrs> {
        AFSEL11_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn afsel12(&mut self) -> AFSEL12_W<GPIO_AFRHrs> {
        AFSEL12_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn afsel13(&mut self) -> AFSEL13_W<GPIO_AFRHrs> {
        AFSEL13_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn afsel14(&mut self) -> AFSEL14_W<GPIO_AFRHrs> {
        AFSEL14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn afsel15(&mut self) -> AFSEL15_W<GPIO_AFRHrs> {
        AFSEL15_W::new(self, 28)
    }
}
#[doc = "GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_afrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_afrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_AFRHrs;
impl crate::RegisterSpec for GPIO_AFRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_afrh::R`](R) reader structure"]
impl crate::Readable for GPIO_AFRHrs {}
#[doc = "`write(|w| ..)` method takes [`gpio_afrh::W`](W) writer structure"]
impl crate::Writable for GPIO_AFRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_AFRH to value 0"]
impl crate::Resettable for GPIO_AFRHrs {
    const RESET_VALUE: u32 = 0;
}
