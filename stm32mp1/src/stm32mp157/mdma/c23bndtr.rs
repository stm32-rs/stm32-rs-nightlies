///Register `C23BNDTR` reader
pub type R = crate::R<C23BNDTRrs>;
///Register `C23BNDTR` writer
pub type W = crate::W<C23BNDTRrs>;
///Field `BNDT` reader - BNDT
pub type BNDT_R = crate::FieldReader<u32>;
///Field `BNDT` writer - BNDT
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
///Field `BRSUM` reader - BRSUM
pub type BRSUM_R = crate::BitReader;
///Field `BRSUM` writer - BRSUM
pub type BRSUM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRDUM` reader - BRDUM
pub type BRDUM_R = crate::BitReader;
///Field `BRDUM` writer - BRDUM
pub type BRDUM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRC` reader - BRC
pub type BRC_R = crate::FieldReader<u16>;
///Field `BRC` writer - BRC
pub type BRC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:16 - BNDT
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new(self.bits & 0x0001_ffff)
    }
    ///Bit 18 - BRSUM
    #[inline(always)]
    pub fn brsum(&self) -> BRSUM_R {
        BRSUM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - BRDUM
    #[inline(always)]
    pub fn brdum(&self) -> BRDUM_R {
        BRDUM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:31 - BRC
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C23BNDTR")
            .field("bndt", &self.bndt())
            .field("brsum", &self.brsum())
            .field("brdum", &self.brdum())
            .field("brc", &self.brc())
            .finish()
    }
}
impl W {
    ///Bits 0:16 - BNDT
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W<C23BNDTRrs> {
        BNDT_W::new(self, 0)
    }
    ///Bit 18 - BRSUM
    #[inline(always)]
    pub fn brsum(&mut self) -> BRSUM_W<C23BNDTRrs> {
        BRSUM_W::new(self, 18)
    }
    ///Bit 19 - BRDUM
    #[inline(always)]
    pub fn brdum(&mut self) -> BRDUM_W<C23BNDTRrs> {
        BRDUM_W::new(self, 19)
    }
    ///Bits 20:31 - BRC
    #[inline(always)]
    pub fn brc(&mut self) -> BRC_W<C23BNDTRrs> {
        BRC_W::new(self, 20)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c23bndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23bndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23BNDTR)*/
pub struct C23BNDTRrs;
impl crate::RegisterSpec for C23BNDTRrs {
    type Ux = u32;
}
///`read()` method returns [`c23bndtr::R`](R) reader structure
impl crate::Readable for C23BNDTRrs {}
///`write(|w| ..)` method takes [`c23bndtr::W`](W) writer structure
impl crate::Writable for C23BNDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C23BNDTR to value 0
impl crate::Resettable for C23BNDTRrs {}
