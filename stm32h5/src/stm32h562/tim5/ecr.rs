#[doc = "Register `ECR` reader"]
pub type R = crate::R<ECRrs>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<ECRrs>;
#[doc = "Field `IE` reader - Index enable This bit indicates if the Index event resets the counter."]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Index enable This bit indicates if the Index event resets the counter."]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDIR` reader - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\]
bitfield must be written when IE bit is reset (index disabled)."]
pub type IDIR_R = crate::FieldReader;
#[doc = "Field `IDIR` writer - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\]
bitfield must be written when IE bit is reset (index disabled)."]
pub type IDIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IBLK` reader - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
pub type IBLK_R = crate::FieldReader;
#[doc = "Field `IBLK` writer - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
pub type IBLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIDX` reader - First index This bit indicates if the first index only is taken into account"]
pub type FIDX_R = crate::BitReader;
#[doc = "Field `FIDX` writer - First index This bit indicates if the first index only is taken into account"]
pub type FIDX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPOS` reader - Index positioning In quadrature encoder mode (SMS\\[3:0\\]
= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\]
= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\]
bit is not significant"]
pub type IPOS_R = crate::FieldReader;
#[doc = "Field `IPOS` writer - Index positioning In quadrature encoder mode (SMS\\[3:0\\]
= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\]
= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\]
bit is not significant"]
pub type IPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PW` reader - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\]
x tPWG"]
pub type PW_R = crate::FieldReader;
#[doc = "Field `PW` writer - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\]
x tPWG"]
pub type PW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWPRSC` reader - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
pub type PWPRSC_R = crate::FieldReader;
#[doc = "Field `PWPRSC` writer - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
pub type PWPRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Index enable This bit indicates if the Index event resets the counter."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\]
bitfield must be written when IE bit is reset (index disabled)."]
    #[inline(always)]
    pub fn idir(&self) -> IDIR_R {
        IDIR_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
    #[inline(always)]
    pub fn iblk(&self) -> IBLK_R {
        IBLK_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - First index This bit indicates if the first index only is taken into account"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Index positioning In quadrature encoder mode (SMS\\[3:0\\]
= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\]
= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\]
bit is not significant"]
    #[inline(always)]
    pub fn ipos(&self) -> IPOS_R {
        IPOS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\]
x tPWG"]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
    #[inline(always)]
    pub fn pwprsc(&self) -> PWPRSC_R {
        PWPRSC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Index enable This bit indicates if the Index event resets the counter."]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<ECRrs> {
        IE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\\[1:0\\]
bitfield must be written when IE bit is reset (index disabled)."]
    #[inline(always)]
    #[must_use]
    pub fn idir(&mut self) -> IDIR_W<ECRrs> {
        IDIR_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input"]
    #[inline(always)]
    #[must_use]
    pub fn iblk(&mut self) -> IBLK_W<ECRrs> {
        IBLK_W::new(self, 3)
    }
    #[doc = "Bit 5 - First index This bit indicates if the first index only is taken into account"]
    #[inline(always)]
    #[must_use]
    pub fn fidx(&mut self) -> FIDX_W<ECRrs> {
        FIDX_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Index positioning In quadrature encoder mode (SMS\\[3:0\\]
= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\\[3:0\\]
= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\\[1\\]
bit is not significant"]
    #[inline(always)]
    #[must_use]
    pub fn ipos(&mut self) -> IPOS_W<ECRrs> {
        IPOS_W::new(self, 6)
    }
    #[doc = "Bits 16:23 - Pulse width This bitfield defines the pulse duration, as following: tPW = PW\\[7:0\\]
x tPWG"]
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<ECRrs> {
        PW_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: tPWG = (2(PWPRSC\\[2:0\\])) x ttim_ker_ck"]
    #[inline(always)]
    #[must_use]
    pub fn pwprsc(&mut self) -> PWPRSC_W<ECRrs> {
        PWPRSC_W::new(self, 24)
    }
}
#[doc = "TIM5 timer encoder control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECRrs;
impl crate::RegisterSpec for ECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for ECRrs {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for ECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECRrs {
    const RESET_VALUE: u32 = 0;
}
