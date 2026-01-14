///Register `BNDTR` reader
pub type R = crate::R<BNDTRrs>;
///Register `BNDTR` writer
pub type W = crate::W<BNDTRrs>;
///Field `BNDT` reader - block number of data to transfer
pub type BNDT_R = crate::FieldReader<u32>;
///Field `BNDT` writer - block number of data to transfer
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
///Field `BRSUM` reader - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0.
pub type BRSUM_R = crate::BitReader;
///Field `BRSUM` writer - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0.
pub type BRSUM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRDUM` reader - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0.
pub type BRDUM_R = crate::BitReader;
///Field `BRDUM` writer - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0.
pub type BRDUM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRC` reader - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0.
pub type BRC_R = crate::FieldReader<u16>;
///Field `BRC` writer - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0.
pub type BRC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:16 - block number of data to transfer
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new(self.bits & 0x0001_ffff)
    }
    ///Bit 18 - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn brsum(&self) -> BRSUM_R {
        BRSUM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn brdum(&self) -> BRDUM_R {
        BRDUM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:31 - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BNDTR")
            .field("bndt", &self.bndt())
            .field("brsum", &self.brsum())
            .field("brdum", &self.brdum())
            .field("brc", &self.brc())
            .finish()
    }
}
impl W {
    ///Bits 0:16 - block number of data to transfer
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W<'_, BNDTRrs> {
        BNDT_W::new(self, 0)
    }
    ///Bit 18 - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn brsum(&mut self) -> BRSUM_W<'_, BNDTRrs> {
        BRSUM_W::new(self, 18)
    }
    ///Bit 19 - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn brdum(&mut self) -> BRDUM_W<'_, BNDTRrs> {
        BRDUM_W::new(self, 19)
    }
    ///Bits 20:31 - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn brc(&mut self) -> BRC_W<'_, BNDTRrs> {
        BRC_W::new(self, 20)
    }
}
/**MDMA Channel x block number of data register

You can [`read`](crate::Reg::read) this register and get [`bndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BNDTRrs;
impl crate::RegisterSpec for BNDTRrs {
    type Ux = u32;
}
///`read()` method returns [`bndtr::R`](R) reader structure
impl crate::Readable for BNDTRrs {}
///`write(|w| ..)` method takes [`bndtr::W`](W) writer structure
impl crate::Writable for BNDTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BNDTR to value 0
impl crate::Resettable for BNDTRrs {}
