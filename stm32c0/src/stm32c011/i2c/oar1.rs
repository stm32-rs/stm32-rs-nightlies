#[doc = "Register `OAR1` reader"]
pub type R = crate::R<OAR1rs>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<OAR1rs>;
#[doc = "Field `OA1` reader - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub type OA1_R = crate::FieldReader<u16>;
#[doc = "Field `OA1` writer - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
pub type OA1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OA1MODE` reader - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub type OA1MODE_R = crate::BitReader;
#[doc = "Field `OA1MODE` writer - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
pub type OA1MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA1EN` reader - Own Address 1 enable"]
pub type OA1EN_R = crate::BitReader;
#[doc = "Field `OA1EN` writer - Own Address 1 enable"]
pub type OA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn oa1(&mut self) -> OA1_W<OAR1rs> {
        OA1_W::new(self, 0)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn oa1mode(&mut self) -> OA1MODE_W<OAR1rs> {
        OA1MODE_W::new(self, 10)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn oa1en(&mut self) -> OA1EN_W<OAR1rs> {
        OA1EN_W::new(self, 15)
    }
}
#[doc = "I2C own address 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OAR1rs;
impl crate::RegisterSpec for OAR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for OAR1rs {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for OAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for OAR1rs {
    const RESET_VALUE: u32 = 0;
}
