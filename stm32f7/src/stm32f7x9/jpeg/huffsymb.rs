#[doc = "Register `HUFFSYMB%s` reader"]
pub type R = crate::R<HUFFSYMBrs>;
#[doc = "Register `HUFFSYMB%s` writer"]
pub type W = crate::W<HUFFSYMBrs>;
#[doc = "Field `HuffSymb_RAM` reader - DHTSymb RAM"]
pub type HUFF_SYMB_RAM_R = crate::FieldReader<u32>;
#[doc = "Field `HuffSymb_RAM` writer - DHTSymb RAM"]
pub type HUFF_SYMB_RAM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DHTSymb RAM"]
    #[inline(always)]
    pub fn huff_symb_ram(&self) -> HUFF_SYMB_RAM_R {
        HUFF_SYMB_RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DHTSymb RAM"]
    #[inline(always)]
    #[must_use]
    pub fn huff_symb_ram(&mut self) -> HUFF_SYMB_RAM_W<HUFFSYMBrs> {
        HUFF_SYMB_RAM_W::new(self, 0)
    }
}
#[doc = "JPEG HUFFSYMB tables\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`huffsymb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`huffsymb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUFFSYMBrs;
impl crate::RegisterSpec for HUFFSYMBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`huffsymb::R`](R) reader structure"]
impl crate::Readable for HUFFSYMBrs {}
#[doc = "`write(|w| ..)` method takes [`huffsymb::W`](W) writer structure"]
impl crate::Writable for HUFFSYMBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HUFFSYMB%s to value 0"]
impl crate::Resettable for HUFFSYMBrs {
    const RESET_VALUE: u32 = 0;
}
