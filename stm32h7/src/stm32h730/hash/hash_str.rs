///Register `HASH_STR` reader
pub type R = crate::R<HASH_STRrs>;
///Register `HASH_STR` writer
pub type W = crate::W<HASH_STRrs>;
///Field `NBLW` reader - Number of valid bits in the last word
pub type NBLW_R = crate::FieldReader;
///Field `NBLW` writer - Number of valid bits in the last word
pub type NBLW_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DCAL` reader - Digest calculation
pub type DCAL_R = crate::BitReader;
///Field `DCAL` writer - Digest calculation
pub type DCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Number of valid bits in the last word
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Digest calculation
    #[inline(always)]
    pub fn dcal(&self) -> DCAL_R {
        DCAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_STR")
            .field("nblw", &self.nblw())
            .field("dcal", &self.dcal())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Number of valid bits in the last word
    #[inline(always)]
    pub fn nblw(&mut self) -> NBLW_W<'_, HASH_STRrs> {
        NBLW_W::new(self, 0)
    }
    ///Bit 8 - Digest calculation
    #[inline(always)]
    pub fn dcal(&mut self) -> DCAL_W<'_, HASH_STRrs> {
        DCAL_W::new(self, 8)
    }
}
/**HASH start register

You can [`read`](crate::Reg::read) this register and get [`hash_str::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_str::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_STR)*/
pub struct HASH_STRrs;
impl crate::RegisterSpec for HASH_STRrs {
    type Ux = u32;
}
///`read()` method returns [`hash_str::R`](R) reader structure
impl crate::Readable for HASH_STRrs {}
///`write(|w| ..)` method takes [`hash_str::W`](W) writer structure
impl crate::Writable for HASH_STRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_STR to value 0
impl crate::Resettable for HASH_STRrs {}
