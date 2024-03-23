#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `ADD` reader - Address of the USART node"]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the USART node"]
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LBDL` reader - lin break detection length"]
pub type LBDL_R = crate::BitReader;
#[doc = "Field `LBDL` writer - lin break detection length"]
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBCL` reader - Last bit clock pulse"]
pub type LBCL_R = crate::BitReader;
#[doc = "Field `LBCL` writer - Last bit clock pulse"]
pub type LBCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type STOP_R = crate::FieldReader;
#[doc = "Field `STOP` writer - STOP bits"]
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LINEN_R = crate::BitReader;
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<CR2rs> {
        ADD_W::new(self, 0)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LBDL_W<CR2rs> {
        LBDL_W::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LBDIE_W<CR2rs> {
        LBDIE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn lbcl(&mut self) -> LBCL_W<CR2rs> {
        LBCL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CR2rs> {
        CPHA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CR2rs> {
        CPOL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CR2rs> {
        CLKEN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LINEN_W<CR2rs> {
        LINEN_W::new(self, 14)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
