#[doc = "Register `HUFFENC_AC0%s` reader"]
pub type R = crate::R<HUFFENC_AC0rs>;
#[doc = "Register `HUFFENC_AC0%s` writer"]
pub type W = crate::W<HUFFENC_AC0rs>;
#[doc = "Field `DHTMem_RAM` reader - DHTMem RAM"]
pub type DHTMEM_RAM_R = crate::FieldReader<u32>;
#[doc = "Field `DHTMem_RAM` writer - DHTMem RAM"]
pub type DHTMEM_RAM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DHTMem RAM"]
    #[inline(always)]
    pub fn dhtmem_ram(&self) -> DHTMEM_RAM_R {
        DHTMEM_RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DHTMem RAM"]
    #[inline(always)]
    #[must_use]
    pub fn dhtmem_ram(&mut self) -> DHTMEM_RAM_W<HUFFENC_AC0rs> {
        DHTMEM_RAM_W::new(self, 0)
    }
}
#[doc = "JPEG encoder, AC Huffman table 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`huffenc_ac0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`huffenc_ac0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUFFENC_AC0rs;
impl crate::RegisterSpec for HUFFENC_AC0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`huffenc_ac0::R`](R) reader structure"]
impl crate::Readable for HUFFENC_AC0rs {}
#[doc = "`write(|w| ..)` method takes [`huffenc_ac0::W`](W) writer structure"]
impl crate::Writable for HUFFENC_AC0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HUFFENC_AC0%s to value 0"]
impl crate::Resettable for HUFFENC_AC0rs {
    const RESET_VALUE: u32 = 0;
}
