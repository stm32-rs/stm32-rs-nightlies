#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `FUNC` reader - Function"]
pub type FUNC_R = crate::FieldReader;
#[doc = "Field `FUNC` writer - Function"]
pub type FUNC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRECISION` reader - Precision required (number of iterations)"]
pub type PRECISION_R = crate::FieldReader;
#[doc = "Field `PRECISION` writer - Precision required (number of iterations)"]
pub type PRECISION_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCALE` reader - Scaling factor"]
pub type SCALE_R = crate::FieldReader;
#[doc = "Field `SCALE` writer - Scaling factor"]
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IEN` reader - Enable interrupt"]
pub type IEN_R = crate::BitReader;
#[doc = "Field `IEN` writer - Enable interrupt"]
pub type IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREN` reader - Enable DMA read channel"]
pub type DMAREN_R = crate::BitReader;
#[doc = "Field `DMAREN` writer - Enable DMA read channel"]
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAWEN` reader - Enable DMA write channel"]
pub type DMAWEN_R = crate::BitReader;
#[doc = "Field `DMAWEN` writer - Enable DMA write channel"]
pub type DMAWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRES` reader - Number of results in the CORDIC_RDATA register"]
pub type NRES_R = crate::BitReader;
#[doc = "Field `NRES` writer - Number of results in the CORDIC_RDATA register"]
pub type NRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NARGS` reader - Number of arguments expected by the CORDIC_WDATA register"]
pub type NARGS_R = crate::BitReader;
#[doc = "Field `NARGS` writer - Number of arguments expected by the CORDIC_WDATA register"]
pub type NARGS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESSIZE` reader - Width of output data"]
pub type RESSIZE_R = crate::BitReader;
#[doc = "Field `RESSIZE` writer - Width of output data"]
pub type RESSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARGSIZE` reader - Width of input data"]
pub type ARGSIZE_R = crate::BitReader;
#[doc = "Field `ARGSIZE` writer - Width of input data"]
pub type ARGSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRDY` reader - Result ready flag"]
pub type RRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Precision required (number of iterations)"]
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Scaling factor"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable interrupt"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable DMA read channel"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable DMA write channel"]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Number of results in the CORDIC_RDATA register"]
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Number of arguments expected by the CORDIC_WDATA register"]
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Width of output data"]
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Width of input data"]
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Result ready flag"]
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<CSRrs> {
        FUNC_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Precision required (number of iterations)"]
    #[inline(always)]
    #[must_use]
    pub fn precision(&mut self) -> PRECISION_W<CSRrs> {
        PRECISION_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Scaling factor"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<CSRrs> {
        SCALE_W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<CSRrs> {
        IEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable DMA read channel"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CSRrs> {
        DMAREN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable DMA write channel"]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<CSRrs> {
        DMAWEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Number of results in the CORDIC_RDATA register"]
    #[inline(always)]
    #[must_use]
    pub fn nres(&mut self) -> NRES_W<CSRrs> {
        NRES_W::new(self, 19)
    }
    #[doc = "Bit 20 - Number of arguments expected by the CORDIC_WDATA register"]
    #[inline(always)]
    #[must_use]
    pub fn nargs(&mut self) -> NARGS_W<CSRrs> {
        NARGS_W::new(self, 20)
    }
    #[doc = "Bit 21 - Width of output data"]
    #[inline(always)]
    #[must_use]
    pub fn ressize(&mut self) -> RESSIZE_W<CSRrs> {
        RESSIZE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Width of input data"]
    #[inline(always)]
    #[must_use]
    pub fn argsize(&mut self) -> ARGSIZE_W<CSRrs> {
        ARGSIZE_W::new(self, 22)
    }
}
#[doc = "CORDIC Control Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x50"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x50;
}
