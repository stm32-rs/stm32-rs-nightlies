///Register `PAGEPROT1` reader
pub type R = crate::R<PAGEPROT1rs>;
///Register `PAGEPROT1` writer
pub type W = crate::W<PAGEPROT1rs>;
///Field `SEGSIZE2` reader - Third segment, 7-bit page protection size (number of pages to protect in segment, first page included)
pub type SEGSIZE2_R = crate::FieldReader;
///Field `SEGSIZE2` writer - Third segment, 7-bit page protection size (number of pages to protect in segment, first page included)
pub type SEGSIZE2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SEGOFFSET2` reader - Third segment, 7-bit page protection offset (first page number in protected segment)
pub type SEGOFFSET2_R = crate::FieldReader;
///Field `SEGOFFSET2` writer - Third segment, 7-bit page protection offset (first page number in protected segment)
pub type SEGOFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SEGSIZE3` reader - Fourth segment, 7-bit page protection size (number of pages to protect in segment, first page included)
pub type SEGSIZE3_R = crate::FieldReader;
///Field `SEGSIZE3` writer - Fourth segment, 7-bit page protection size (number of pages to protect in segment, first page included)
pub type SEGSIZE3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SEGOFFSET3` reader - Fourth segment, 7-bit page protection offset (first page number in protected segment)
pub type SEGOFFSET3_R = crate::FieldReader;
///Field `SEGOFFSET3` writer - Fourth segment, 7-bit page protection offset (first page number in protected segment)
pub type SEGOFFSET3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Third segment, 7-bit page protection size (number of pages to protect in segment, first page included)
    #[inline(always)]
    pub fn segsize2(&self) -> SEGSIZE2_R {
        SEGSIZE2_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - Third segment, 7-bit page protection offset (first page number in protected segment)
    #[inline(always)]
    pub fn segoffset2(&self) -> SEGOFFSET2_R {
        SEGOFFSET2_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 16:22 - Fourth segment, 7-bit page protection size (number of pages to protect in segment, first page included)
    #[inline(always)]
    pub fn segsize3(&self) -> SEGSIZE3_R {
        SEGSIZE3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - Fourth segment, 7-bit page protection offset (first page number in protected segment)
    #[inline(always)]
    pub fn segoffset3(&self) -> SEGOFFSET3_R {
        SEGOFFSET3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAGEPROT1")
            .field("segsize2", &self.segsize2())
            .field("segoffset2", &self.segoffset2())
            .field("segsize3", &self.segsize3())
            .field("segoffset3", &self.segoffset3())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Third segment, 7-bit page protection size (number of pages to protect in segment, first page included)
    #[inline(always)]
    pub fn segsize2(&mut self) -> SEGSIZE2_W<'_, PAGEPROT1rs> {
        SEGSIZE2_W::new(self, 0)
    }
    ///Bits 8:14 - Third segment, 7-bit page protection offset (first page number in protected segment)
    #[inline(always)]
    pub fn segoffset2(&mut self) -> SEGOFFSET2_W<'_, PAGEPROT1rs> {
        SEGOFFSET2_W::new(self, 8)
    }
    ///Bits 16:22 - Fourth segment, 7-bit page protection size (number of pages to protect in segment, first page included)
    #[inline(always)]
    pub fn segsize3(&mut self) -> SEGSIZE3_W<'_, PAGEPROT1rs> {
        SEGSIZE3_W::new(self, 16)
    }
    ///Bits 24:30 - Fourth segment, 7-bit page protection offset (first page number in protected segment)
    #[inline(always)]
    pub fn segoffset3(&mut self) -> SEGOFFSET3_W<'_, PAGEPROT1rs> {
        SEGOFFSET3_W::new(self, 24)
    }
}
/**PAGEPROT1 register

You can [`read`](crate::Reg::read) this register and get [`pageprot1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#FLASH_CTRL:PAGEPROT1)*/
pub struct PAGEPROT1rs;
impl crate::RegisterSpec for PAGEPROT1rs {
    type Ux = u32;
}
///`read()` method returns [`pageprot1::R`](R) reader structure
impl crate::Readable for PAGEPROT1rs {}
///`write(|w| ..)` method takes [`pageprot1::W`](W) writer structure
impl crate::Writable for PAGEPROT1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PAGEPROT1 to value 0
impl crate::Resettable for PAGEPROT1rs {}
