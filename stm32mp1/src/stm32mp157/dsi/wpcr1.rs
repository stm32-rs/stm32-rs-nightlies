///Register `WPCR1` reader
pub type R = crate::R<WPCR1rs>;
///Register `WPCR1` writer
pub type W = crate::W<WPCR1rs>;
///Field `SKEWCL` reader - SKEWCL
pub type SKEWCL_R = crate::FieldReader;
///Field `SKEWCL` writer - SKEWCL
pub type SKEWCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SKEWDL` reader - SKEWDL
pub type SKEWDL_R = crate::FieldReader;
///Field `SKEWDL` writer - SKEWDL
pub type SKEWDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTXSRCL` reader - LPTXSRCL
pub type LPTXSRCL_R = crate::FieldReader;
///Field `LPTXSRCL` writer - LPTXSRCL
pub type LPTXSRCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTXSRDL` reader - LPTXSRDL
pub type LPTXSRDL_R = crate::FieldReader;
///Field `LPTXSRDL` writer - LPTXSRDL
pub type LPTXSRDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SDDCCL` reader - SDDCCL
pub type SDDCCL_R = crate::BitReader;
///Field `SDDCCL` writer - SDDCCL
pub type SDDCCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDDCDL` reader - SDDCDL
pub type SDDCDL_R = crate::BitReader;
///Field `SDDCDL` writer - SDDCDL
pub type SDDCDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSTXSRUCL` reader - HSTXSRUCL
pub type HSTXSRUCL_R = crate::BitReader;
///Field `HSTXSRUCL` writer - HSTXSRUCL
pub type HSTXSRUCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSTXSRDCL` reader - HSTXSRDCL
pub type HSTXSRDCL_R = crate::BitReader;
///Field `HSTXSRDCL` writer - HSTXSRDCL
pub type HSTXSRDCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSTXSRUDL` reader - HSTXSRUDL
pub type HSTXSRUDL_R = crate::BitReader;
///Field `HSTXSRUDL` writer - HSTXSRUDL
pub type HSTXSRUDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSTXSRDDL` reader - HSTXSRDDL
pub type HSTXSRDDL_R = crate::BitReader;
///Field `HSTXSRDDL` writer - HSTXSRDDL
pub type HSTXSRDDL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - SKEWCL
    #[inline(always)]
    pub fn skewcl(&self) -> SKEWCL_R {
        SKEWCL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - SKEWDL
    #[inline(always)]
    pub fn skewdl(&self) -> SKEWDL_R {
        SKEWDL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 6:7 - LPTXSRCL
    #[inline(always)]
    pub fn lptxsrcl(&self) -> LPTXSRCL_R {
        LPTXSRCL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - LPTXSRDL
    #[inline(always)]
    pub fn lptxsrdl(&self) -> LPTXSRDL_R {
        LPTXSRDL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 12 - SDDCCL
    #[inline(always)]
    pub fn sddccl(&self) -> SDDCCL_R {
        SDDCCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SDDCDL
    #[inline(always)]
    pub fn sddcdl(&self) -> SDDCDL_R {
        SDDCDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - HSTXSRUCL
    #[inline(always)]
    pub fn hstxsrucl(&self) -> HSTXSRUCL_R {
        HSTXSRUCL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSTXSRDCL
    #[inline(always)]
    pub fn hstxsrdcl(&self) -> HSTXSRDCL_R {
        HSTXSRDCL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSTXSRUDL
    #[inline(always)]
    pub fn hstxsrudl(&self) -> HSTXSRUDL_R {
        HSTXSRUDL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HSTXSRDDL
    #[inline(always)]
    pub fn hstxsrddl(&self) -> HSTXSRDDL_R {
        HSTXSRDDL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR1")
            .field("skewcl", &self.skewcl())
            .field("skewdl", &self.skewdl())
            .field("lptxsrcl", &self.lptxsrcl())
            .field("lptxsrdl", &self.lptxsrdl())
            .field("sddccl", &self.sddccl())
            .field("sddcdl", &self.sddcdl())
            .field("hstxsrucl", &self.hstxsrucl())
            .field("hstxsrdcl", &self.hstxsrdcl())
            .field("hstxsrudl", &self.hstxsrudl())
            .field("hstxsrddl", &self.hstxsrddl())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SKEWCL
    #[inline(always)]
    pub fn skewcl(&mut self) -> SKEWCL_W<'_, WPCR1rs> {
        SKEWCL_W::new(self, 0)
    }
    ///Bits 2:3 - SKEWDL
    #[inline(always)]
    pub fn skewdl(&mut self) -> SKEWDL_W<'_, WPCR1rs> {
        SKEWDL_W::new(self, 2)
    }
    ///Bits 6:7 - LPTXSRCL
    #[inline(always)]
    pub fn lptxsrcl(&mut self) -> LPTXSRCL_W<'_, WPCR1rs> {
        LPTXSRCL_W::new(self, 6)
    }
    ///Bits 8:9 - LPTXSRDL
    #[inline(always)]
    pub fn lptxsrdl(&mut self) -> LPTXSRDL_W<'_, WPCR1rs> {
        LPTXSRDL_W::new(self, 8)
    }
    ///Bit 12 - SDDCCL
    #[inline(always)]
    pub fn sddccl(&mut self) -> SDDCCL_W<'_, WPCR1rs> {
        SDDCCL_W::new(self, 12)
    }
    ///Bit 13 - SDDCDL
    #[inline(always)]
    pub fn sddcdl(&mut self) -> SDDCDL_W<'_, WPCR1rs> {
        SDDCDL_W::new(self, 13)
    }
    ///Bit 16 - HSTXSRUCL
    #[inline(always)]
    pub fn hstxsrucl(&mut self) -> HSTXSRUCL_W<'_, WPCR1rs> {
        HSTXSRUCL_W::new(self, 16)
    }
    ///Bit 17 - HSTXSRDCL
    #[inline(always)]
    pub fn hstxsrdcl(&mut self) -> HSTXSRDCL_W<'_, WPCR1rs> {
        HSTXSRDCL_W::new(self, 17)
    }
    ///Bit 18 - HSTXSRUDL
    #[inline(always)]
    pub fn hstxsrudl(&mut self) -> HSTXSRUDL_W<'_, WPCR1rs> {
        HSTXSRUDL_W::new(self, 18)
    }
    ///Bit 19 - HSTXSRDDL
    #[inline(always)]
    pub fn hstxsrddl(&mut self) -> HSTXSRDDL_W<'_, WPCR1rs> {
        HSTXSRDDL_W::new(self, 19)
    }
}
/**This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).

You can [`read`](crate::Reg::read) this register and get [`wpcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DSI:WPCR1)*/
pub struct WPCR1rs;
impl crate::RegisterSpec for WPCR1rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr1::R`](R) reader structure
impl crate::Readable for WPCR1rs {}
///`write(|w| ..)` method takes [`wpcr1::W`](W) writer structure
impl crate::Writable for WPCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR1 to value 0
impl crate::Resettable for WPCR1rs {}
