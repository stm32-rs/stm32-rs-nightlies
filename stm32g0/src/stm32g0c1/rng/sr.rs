#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `DRDY` reader - Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1."]
pub type DRDY_R = crate::BitReader;
#[doc = "Field `CECS` reader - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0."]
pub type CECS_R = crate::BitReader;
#[doc = "Field `SECS` reader - Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 32 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d}) Both noise sources have delivered more than 32 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 16 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d})"]
pub type SECS_R = crate::BitReader;
#[doc = "Field `CEIS` reader - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type CEIS_R = crate::BitReader;
#[doc = "Field `CEIS` writer - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type CEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEIS` reader - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type SEIS_R = crate::BitReader;
#[doc = "Field `SEIS` writer - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
pub type SEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1."]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0."]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 32 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d}) Both noise sources have delivered more than 32 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 16 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d})"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    #[must_use]
    pub fn ceis(&mut self) -> CEIS_W<SRrs> {
        CEIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    #[must_use]
    pub fn seis(&mut self) -> SEIS_W<SRrs> {
        SEIS_W::new(self, 6)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
