///Register `L2FPF1R` reader
pub type R = crate::R<L2FPF1Rrs>;
///Register `L2FPF1R` writer
pub type W = crate::W<L2FPF1Rrs>;
///Field `GPOS` reader - Location of the Green component inside the pixel memory word (in bits)
pub type GPOS_R = crate::FieldReader;
///Field `GPOS` writer - Location of the Green component inside the pixel memory word (in bits)
pub type GPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `GLEN` reader - Width of the Green component (in bits)
pub type GLEN_R = crate::FieldReader;
///Field `GLEN` writer - Width of the Green component (in bits)
pub type GLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BPOS` reader - Location of the Blue component inside the pixel memory word (in bits)
pub type BPOS_R = crate::FieldReader;
///Field `BPOS` writer - Location of the Blue component inside the pixel memory word (in bits)
pub type BPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BLEN` reader - Width of the Blue component (in bits)
pub type BLEN_R = crate::FieldReader;
///Field `BLEN` writer - Width of the Blue component (in bits)
pub type BLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PSIZE` reader - Pixel size (in bytes)
pub type PSIZE_R = crate::FieldReader;
///Field `PSIZE` writer - Pixel size (in bytes)
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:4 - Location of the Green component inside the pixel memory word (in bits)
    #[inline(always)]
    pub fn gpos(&self) -> GPOS_R {
        GPOS_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:8 - Width of the Green component (in bits)
    #[inline(always)]
    pub fn glen(&self) -> GLEN_R {
        GLEN_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:13 - Location of the Blue component inside the pixel memory word (in bits)
    #[inline(always)]
    pub fn bpos(&self) -> BPOS_R {
        BPOS_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    ///Bits 14:17 - Width of the Blue component (in bits)
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bits 18:20 - Pixel size (in bytes)
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2FPF1R")
            .field("gpos", &self.gpos())
            .field("glen", &self.glen())
            .field("bpos", &self.bpos())
            .field("blen", &self.blen())
            .field("psize", &self.psize())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Location of the Green component inside the pixel memory word (in bits)
    #[inline(always)]
    pub fn gpos(&mut self) -> GPOS_W<'_, L2FPF1Rrs> {
        GPOS_W::new(self, 0)
    }
    ///Bits 5:8 - Width of the Green component (in bits)
    #[inline(always)]
    pub fn glen(&mut self) -> GLEN_W<'_, L2FPF1Rrs> {
        GLEN_W::new(self, 5)
    }
    ///Bits 9:13 - Location of the Blue component inside the pixel memory word (in bits)
    #[inline(always)]
    pub fn bpos(&mut self) -> BPOS_W<'_, L2FPF1Rrs> {
        BPOS_W::new(self, 9)
    }
    ///Bits 14:17 - Width of the Blue component (in bits)
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W<'_, L2FPF1Rrs> {
        BLEN_W::new(self, 14)
    }
    ///Bits 18:20 - Pixel size (in bytes)
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, L2FPF1Rrs> {
        PSIZE_W::new(self, 18)
    }
}
/**LTDC layerx flexible pixel format 1 register

You can [`read`](crate::Reg::read) this register and get [`l2fpf1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2fpf1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:L2FPF1R)*/
pub struct L2FPF1Rrs;
impl crate::RegisterSpec for L2FPF1Rrs {
    type Ux = u32;
}
///`read()` method returns [`l2fpf1r::R`](R) reader structure
impl crate::Readable for L2FPF1Rrs {}
///`write(|w| ..)` method takes [`l2fpf1r::W`](W) writer structure
impl crate::Writable for L2FPF1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2FPF1R to value 0x0009_3110
impl crate::Resettable for L2FPF1Rrs {
    const RESET_VALUE: u32 = 0x0009_3110;
}
