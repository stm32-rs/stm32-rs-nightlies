#[doc = "Register `SQR1` reader"]
pub type R = crate::R<SQR1rs>;
#[doc = "Register `SQR1` writer"]
pub type W = crate::W<SQR1rs>;
#[doc = "Field `SQ25` reader - 25th conversion in regular sequence"]
pub type SQ25_R = crate::FieldReader;
#[doc = "Field `SQ25` writer - 25th conversion in regular sequence"]
pub type SQ25_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ26` reader - 26th conversion in regular sequence"]
pub type SQ26_R = crate::FieldReader;
#[doc = "Field `SQ26` writer - 26th conversion in regular sequence"]
pub type SQ26_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ27` reader - 27th conversion in regular sequence"]
pub type SQ27_R = crate::FieldReader;
#[doc = "Field `SQ27` writer - 27th conversion in regular sequence"]
pub type SQ27_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ28` reader - 28th conversion in regular sequence"]
pub type SQ28_R = crate::FieldReader;
#[doc = "Field `SQ28` writer - 28th conversion in regular sequence"]
pub type SQ28_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L` reader - Regular channel sequence length"]
pub type L_R = crate::FieldReader;
#[doc = "Field `L` writer - Regular channel sequence length"]
pub type L_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 25th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq25(&self) -> SQ25_R {
        SQ25_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 26th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq26(&self) -> SQ26_R {
        SQ26_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 27th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq27(&self) -> SQ27_R {
        SQ27_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 28th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq28(&self) -> SQ28_R {
        SQ28_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 25th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq25(&mut self) -> SQ25_W<SQR1rs> {
        SQ25_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 26th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq26(&mut self) -> SQ26_W<SQR1rs> {
        SQ26_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 27th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq27(&mut self) -> SQ27_W<SQR1rs> {
        SQ27_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 28th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq28(&mut self) -> SQ28_W<SQR1rs> {
        SQ28_W::new(self, 15)
    }
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<SQR1rs> {
        L_W::new(self, 20)
    }
}
#[doc = "regular sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR1rs;
impl crate::RegisterSpec for SQR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr1::R`](R) reader structure"]
impl crate::Readable for SQR1rs {}
#[doc = "`write(|w| ..)` method takes [`sqr1::W`](W) writer structure"]
impl crate::Writable for SQR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for SQR1rs {
    const RESET_VALUE: u32 = 0;
}
