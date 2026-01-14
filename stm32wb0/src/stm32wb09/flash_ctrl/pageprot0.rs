///Register `PAGEPROT0` reader
pub type R = crate::R<PAGEPROT0rs>;
///Register `PAGEPROT0` writer
pub type W = crate::W<PAGEPROT0rs>;
///Field `SEGSIZE0` reader - First segment, 7-bit page protection size (number of pages to protect in segment, first page included)
pub type SEGSIZE0_R = crate::FieldReader;
///Field `SEGSIZE0` writer - First segment, 7-bit page protection size (number of pages to protect in segment, first page included)
pub type SEGSIZE0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SEGOFFSET0` reader - First segment, 7-bit page protection offset (first page number in protected segment)
pub type SEGOFFSET0_R = crate::FieldReader;
///Field `SEGOFFSET0` writer - First segment, 7-bit page protection offset (first page number in protected segment)
pub type SEGOFFSET0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SEGSIZE1` reader - Second segment, 7-bit page protection size (number of pages to protect in segment, first page included)
pub type SEGSIZE1_R = crate::FieldReader;
///Field `SEGSIZE1` writer - Second segment, 7-bit page protection size (number of pages to protect in segment, first page included)
pub type SEGSIZE1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SEGOFFSET1` reader - Second segment, 7-bit page protection offset (first page number in protected segment)
pub type SEGOFFSET1_R = crate::FieldReader;
///Field `SEGOFFSET1` writer - Second segment, 7-bit page protection offset (first page number in protected segment)
pub type SEGOFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - First segment, 7-bit page protection size (number of pages to protect in segment, first page included)
    #[inline(always)]
    pub fn segsize0(&self) -> SEGSIZE0_R {
        SEGSIZE0_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - First segment, 7-bit page protection offset (first page number in protected segment)
    #[inline(always)]
    pub fn segoffset0(&self) -> SEGOFFSET0_R {
        SEGOFFSET0_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 16:22 - Second segment, 7-bit page protection size (number of pages to protect in segment, first page included)
    #[inline(always)]
    pub fn segsize1(&self) -> SEGSIZE1_R {
        SEGSIZE1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - Second segment, 7-bit page protection offset (first page number in protected segment)
    #[inline(always)]
    pub fn segoffset1(&self) -> SEGOFFSET1_R {
        SEGOFFSET1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAGEPROT0")
            .field("segsize0", &self.segsize0())
            .field("segoffset0", &self.segoffset0())
            .field("segsize1", &self.segsize1())
            .field("segoffset1", &self.segoffset1())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - First segment, 7-bit page protection size (number of pages to protect in segment, first page included)
    #[inline(always)]
    pub fn segsize0(&mut self) -> SEGSIZE0_W<'_, PAGEPROT0rs> {
        SEGSIZE0_W::new(self, 0)
    }
    ///Bits 8:14 - First segment, 7-bit page protection offset (first page number in protected segment)
    #[inline(always)]
    pub fn segoffset0(&mut self) -> SEGOFFSET0_W<'_, PAGEPROT0rs> {
        SEGOFFSET0_W::new(self, 8)
    }
    ///Bits 16:22 - Second segment, 7-bit page protection size (number of pages to protect in segment, first page included)
    #[inline(always)]
    pub fn segsize1(&mut self) -> SEGSIZE1_W<'_, PAGEPROT0rs> {
        SEGSIZE1_W::new(self, 16)
    }
    ///Bits 24:30 - Second segment, 7-bit page protection offset (first page number in protected segment)
    #[inline(always)]
    pub fn segoffset1(&mut self) -> SEGOFFSET1_W<'_, PAGEPROT0rs> {
        SEGOFFSET1_W::new(self, 24)
    }
}
/**PAGEPROT0 register

You can [`read`](crate::Reg::read) this register and get [`pageprot0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#FLASH_CTRL:PAGEPROT0)*/
pub struct PAGEPROT0rs;
impl crate::RegisterSpec for PAGEPROT0rs {
    type Ux = u32;
}
///`read()` method returns [`pageprot0::R`](R) reader structure
impl crate::Readable for PAGEPROT0rs {}
///`write(|w| ..)` method takes [`pageprot0::W`](W) writer structure
impl crate::Writable for PAGEPROT0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PAGEPROT0 to value 0
impl crate::Resettable for PAGEPROT0rs {}
