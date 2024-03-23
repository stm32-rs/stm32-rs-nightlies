#[doc = "Register `I2C_OAR2` reader"]
pub type R = crate::R<I2C_OAR2rs>;
#[doc = "Register `I2C_OAR2` writer"]
pub type W = crate::W<I2C_OAR2rs>;
#[doc = "Field `OA2` reader - OA2"]
pub type OA2_R = crate::FieldReader;
#[doc = "Field `OA2` writer - OA2"]
pub type OA2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OA2MSK` reader - OA2MSK"]
pub type OA2MSK_R = crate::FieldReader;
#[doc = "Field `OA2MSK` writer - OA2MSK"]
pub type OA2MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OA2EN` reader - OA2EN"]
pub type OA2EN_R = crate::BitReader;
#[doc = "Field `OA2EN` writer - OA2EN"]
pub type OA2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:7 - OA2"]
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - OA2MSK"]
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - OA2EN"]
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - OA2"]
    #[inline(always)]
    #[must_use]
    pub fn oa2(&mut self) -> OA2_W<I2C_OAR2rs> {
        OA2_W::new(self, 1)
    }
    #[doc = "Bits 8:10 - OA2MSK"]
    #[inline(always)]
    #[must_use]
    pub fn oa2msk(&mut self) -> OA2MSK_W<I2C_OAR2rs> {
        OA2MSK_W::new(self, 8)
    }
    #[doc = "Bit 15 - OA2EN"]
    #[inline(always)]
    #[must_use]
    pub fn oa2en(&mut self) -> OA2EN_W<I2C_OAR2rs> {
        OA2EN_W::new(self, 15)
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_OAR2rs;
impl crate::RegisterSpec for I2C_OAR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_oar2::R`](R) reader structure"]
impl crate::Readable for I2C_OAR2rs {}
#[doc = "`write(|w| ..)` method takes [`i2c_oar2::W`](W) writer structure"]
impl crate::Writable for I2C_OAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_OAR2 to value 0"]
impl crate::Resettable for I2C_OAR2rs {
    const RESET_VALUE: u32 = 0;
}
