#[doc = "Register `DLR` reader"]
pub type R = crate::R<DLRrs>;
#[doc = "Register `DLR` writer"]
pub type W = crate::W<DLRrs>;
#[doc = "Field `DL` reader - 31: 0\\]: Data length Number of data to be retrieved (value+1) in Indirect and Automatic status-polling modes. A value not greater than three (indicating 4 bytes) must be used for Automatic status-polling mode. All 1’s in Indirect mode means undefined length, where OCTOSPI continues until the end of the memory, as defined by DEVSIZE. 0x0000_0000: 1 byte is to be transferred. 0x0000_0001: 2 bytes are to be transferred. 0x0000_0002: 3 bytes are to be transferred. 0x0000_0003: 4 bytes are to be transferred. ... 0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred. 0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred. 0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE = 0x1F. DL\\[0\\]
is stuck at 1 in dual-memory configuration (DMM = 1) even when 0 is written to this bit, thus assuring that each access transfers an even number of bytes. This field has no effect in Memory-mapped mode."]
pub type DL_R = crate::FieldReader<u32>;
#[doc = "Field `DL` writer - 31: 0\\]: Data length Number of data to be retrieved (value+1) in Indirect and Automatic status-polling modes. A value not greater than three (indicating 4 bytes) must be used for Automatic status-polling mode. All 1’s in Indirect mode means undefined length, where OCTOSPI continues until the end of the memory, as defined by DEVSIZE. 0x0000_0000: 1 byte is to be transferred. 0x0000_0001: 2 bytes are to be transferred. 0x0000_0002: 3 bytes are to be transferred. 0x0000_0003: 4 bytes are to be transferred. ... 0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred. 0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred. 0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE = 0x1F. DL\\[0\\]
is stuck at 1 in dual-memory configuration (DMM = 1) even when 0 is written to this bit, thus assuring that each access transfers an even number of bytes. This field has no effect in Memory-mapped mode."]
pub type DL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31: 0\\]: Data length Number of data to be retrieved (value+1) in Indirect and Automatic status-polling modes. A value not greater than three (indicating 4 bytes) must be used for Automatic status-polling mode. All 1’s in Indirect mode means undefined length, where OCTOSPI continues until the end of the memory, as defined by DEVSIZE. 0x0000_0000: 1 byte is to be transferred. 0x0000_0001: 2 bytes are to be transferred. 0x0000_0002: 3 bytes are to be transferred. 0x0000_0003: 4 bytes are to be transferred. ... 0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred. 0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred. 0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE = 0x1F. DL\\[0\\]
is stuck at 1 in dual-memory configuration (DMM = 1) even when 0 is written to this bit, thus assuring that each access transfers an even number of bytes. This field has no effect in Memory-mapped mode."]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31: 0\\]: Data length Number of data to be retrieved (value+1) in Indirect and Automatic status-polling modes. A value not greater than three (indicating 4 bytes) must be used for Automatic status-polling mode. All 1’s in Indirect mode means undefined length, where OCTOSPI continues until the end of the memory, as defined by DEVSIZE. 0x0000_0000: 1 byte is to be transferred. 0x0000_0001: 2 bytes are to be transferred. 0x0000_0002: 3 bytes are to be transferred. 0x0000_0003: 4 bytes are to be transferred. ... 0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred. 0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred. 0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE = 0x1F. DL\\[0\\]
is stuck at 1 in dual-memory configuration (DMM = 1) even when 0 is written to this bit, thus assuring that each access transfers an even number of bytes. This field has no effect in Memory-mapped mode."]
    #[inline(always)]
    #[must_use]
    pub fn dl(&mut self) -> DL_W<DLRrs> {
        DL_W::new(self, 0)
    }
}
#[doc = "OCTOSPI data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLRrs;
impl crate::RegisterSpec for DLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlr::R`](R) reader structure"]
impl crate::Readable for DLRrs {}
#[doc = "`write(|w| ..)` method takes [`dlr::W`](W) writer structure"]
impl crate::Writable for DLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLR to value 0"]
impl crate::Resettable for DLRrs {
    const RESET_VALUE: u32 = 0;
}
