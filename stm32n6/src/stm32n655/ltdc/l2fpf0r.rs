///Register `L2FPF0R` reader
pub type R = crate::R<L2FPF0Rrs>;
///Register `L2FPF0R` writer
pub type W = crate::W<L2FPF0Rrs>;
///Field `APOS` reader - Location of the Alpha component inside the pixel memory word (in bits)
pub type APOS_R = crate::FieldReader;
///Field `APOS` writer - Location of the Alpha component inside the pixel memory word (in bits)
pub type APOS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ALEN` reader - Width of the Alpha component (in bits)
pub type ALEN_R = crate::FieldReader;
///Field `ALEN` writer - Width of the Alpha component (in bits)
pub type ALEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RPOS` reader - Location of the Red component inside the pixel memory word (in bits)
pub type RPOS_R = crate::FieldReader;
///Field `RPOS` writer - Location of the Red component inside the pixel memory word (in bits)
pub type RPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RLEN` reader - Width of the Red component (in bits)
pub type RLEN_R = crate::FieldReader;
///Field `RLEN` writer - Width of the Red component (in bits)
pub type RLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:4 - Location of the Alpha component inside the pixel memory word (in bits)
    #[inline(always)]
    pub fn apos(&self) -> APOS_R {
        APOS_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:8 - Width of the Alpha component (in bits)
    #[inline(always)]
    pub fn alen(&self) -> ALEN_R {
        ALEN_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:13 - Location of the Red component inside the pixel memory word (in bits)
    #[inline(always)]
    pub fn rpos(&self) -> RPOS_R {
        RPOS_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    ///Bits 14:17 - Width of the Red component (in bits)
    #[inline(always)]
    pub fn rlen(&self) -> RLEN_R {
        RLEN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2FPF0R")
            .field("apos", &self.apos())
            .field("alen", &self.alen())
            .field("rpos", &self.rpos())
            .field("rlen", &self.rlen())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Location of the Alpha component inside the pixel memory word (in bits)
    #[inline(always)]
    pub fn apos(&mut self) -> APOS_W<'_, L2FPF0Rrs> {
        APOS_W::new(self, 0)
    }
    ///Bits 5:8 - Width of the Alpha component (in bits)
    #[inline(always)]
    pub fn alen(&mut self) -> ALEN_W<'_, L2FPF0Rrs> {
        ALEN_W::new(self, 5)
    }
    ///Bits 9:13 - Location of the Red component inside the pixel memory word (in bits)
    #[inline(always)]
    pub fn rpos(&mut self) -> RPOS_W<'_, L2FPF0Rrs> {
        RPOS_W::new(self, 9)
    }
    ///Bits 14:17 - Width of the Red component (in bits)
    #[inline(always)]
    pub fn rlen(&mut self) -> RLEN_W<'_, L2FPF0Rrs> {
        RLEN_W::new(self, 14)
    }
}
/**LTDC layerx flexible pixel format 0 register

You can [`read`](crate::Reg::read) this register and get [`l2fpf0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2fpf0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:L2FPF0R)*/
pub struct L2FPF0Rrs;
impl crate::RegisterSpec for L2FPF0Rrs {
    type Ux = u32;
}
///`read()` method returns [`l2fpf0r::R`](R) reader structure
impl crate::Readable for L2FPF0Rrs {}
///`write(|w| ..)` method takes [`l2fpf0r::W`](W) writer structure
impl crate::Writable for L2FPF0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2FPF0R to value 0x0001_1100
impl crate::Resettable for L2FPF0Rrs {
    const RESET_VALUE: u32 = 0x0001_1100;
}
