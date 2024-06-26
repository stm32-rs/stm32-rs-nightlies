///Register `CRC_CR` reader
pub type R = crate::R<CRC_CRrs>;
///Register `CRC_CR` writer
pub type W = crate::W<CRC_CRrs>;
///Field `RESET` reader - RESET
pub type RESET_R = crate::BitReader;
///Field `RESET` writer - RESET
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POLYSIZE` reader - POLYSIZE
pub type POLYSIZE_R = crate::FieldReader;
///Field `POLYSIZE` writer - POLYSIZE
pub type POLYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REV_IN` reader - REV_IN
pub type REV_IN_R = crate::FieldReader;
///Field `REV_IN` writer - REV_IN
pub type REV_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REV_OUT` reader - REV_OUT
pub type REV_OUT_R = crate::BitReader;
///Field `REV_OUT` writer - REV_OUT
pub type REV_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RESET
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    ///Bits 3:4 - POLYSIZE
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - REV_IN
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - REV_OUT
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_CR")
            .field("reset", &self.reset())
            .field("polysize", &self.polysize())
            .field("rev_in", &self.rev_in())
            .field("rev_out", &self.rev_out())
            .finish()
    }
}
impl W {
    ///Bit 0 - RESET
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CRC_CRrs> {
        RESET_W::new(self, 0)
    }
    ///Bits 3:4 - POLYSIZE
    #[inline(always)]
    #[must_use]
    pub fn polysize(&mut self) -> POLYSIZE_W<CRC_CRrs> {
        POLYSIZE_W::new(self, 3)
    }
    ///Bits 5:6 - REV_IN
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> REV_IN_W<CRC_CRrs> {
        REV_IN_W::new(self, 5)
    }
    ///Bit 7 - REV_OUT
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> REV_OUT_W<CRC_CRrs> {
        REV_OUT_W::new(self, 7)
    }
}
/**CRC control register

You can [`read`](crate::Reg::read) this register and get [`crc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRC1:CRC_CR)*/
pub struct CRC_CRrs;
impl crate::RegisterSpec for CRC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`crc_cr::R`](R) reader structure
impl crate::Readable for CRC_CRrs {}
///`write(|w| ..)` method takes [`crc_cr::W`](W) writer structure
impl crate::Writable for CRC_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CRC_CR to value 0
impl crate::Resettable for CRC_CRrs {
    const RESET_VALUE: u32 = 0;
}
