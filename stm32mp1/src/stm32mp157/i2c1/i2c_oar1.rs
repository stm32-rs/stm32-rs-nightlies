#[doc = "Register `I2C_OAR1` reader"]
pub type R = crate::R<I2C_OAR1rs>;
#[doc = "Register `I2C_OAR1` writer"]
pub type W = crate::W<I2C_OAR1rs>;
#[doc = "Field `OA1` reader - OA1"]
pub type OA1_R = crate::FieldReader<u16>;
#[doc = "Field `OA1` writer - OA1"]
pub type OA1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OA1MODE` reader - OA1MODE"]
pub type OA1MODE_R = crate::BitReader;
#[doc = "Field `OA1MODE` writer - OA1MODE"]
pub type OA1MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA1EN` reader - OA1EN"]
pub type OA1EN_R = crate::BitReader;
#[doc = "Field `OA1EN` writer - OA1EN"]
pub type OA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - OA1"]
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - OA1MODE"]
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - OA1EN"]
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - OA1"]
    #[inline(always)]
    #[must_use]
    pub fn oa1(&mut self) -> OA1_W<I2C_OAR1rs> {
        OA1_W::new(self, 0)
    }
    #[doc = "Bit 10 - OA1MODE"]
    #[inline(always)]
    #[must_use]
    pub fn oa1mode(&mut self) -> OA1MODE_W<I2C_OAR1rs> {
        OA1MODE_W::new(self, 10)
    }
    #[doc = "Bit 15 - OA1EN"]
    #[inline(always)]
    #[must_use]
    pub fn oa1en(&mut self) -> OA1EN_W<I2C_OAR1rs> {
        OA1EN_W::new(self, 15)
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_OAR1rs;
impl crate::RegisterSpec for I2C_OAR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_oar1::R`](R) reader structure"]
impl crate::Readable for I2C_OAR1rs {}
#[doc = "`write(|w| ..)` method takes [`i2c_oar1::W`](W) writer structure"]
impl crate::Writable for I2C_OAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_OAR1 to value 0"]
impl crate::Resettable for I2C_OAR1rs {
    const RESET_VALUE: u32 = 0;
}
