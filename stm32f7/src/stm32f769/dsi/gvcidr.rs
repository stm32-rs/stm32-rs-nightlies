///Register `GVCIDR` reader
pub type R = crate::R<GVCIDRrs>;
///Register `GVCIDR` writer
pub type W = crate::W<GVCIDRrs>;
///Field `VCID` reader - Virtual Channel ID
pub type VCID_R = crate::FieldReader;
///Field `VCID` writer - Virtual Channel ID
pub type VCID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Virtual Channel ID
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GVCIDR")
            .field("vcid", &self.vcid())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Virtual Channel ID
    #[inline(always)]
    pub fn vcid(&mut self) -> VCID_W<'_, GVCIDRrs> {
        VCID_W::new(self, 0)
    }
}
/**DSI Host Generic VCID Register

You can [`read`](crate::Reg::read) this register and get [`gvcidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gvcidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:GVCIDR)*/
pub struct GVCIDRrs;
impl crate::RegisterSpec for GVCIDRrs {
    type Ux = u32;
}
///`read()` method returns [`gvcidr::R`](R) reader structure
impl crate::Readable for GVCIDRrs {}
///`write(|w| ..)` method takes [`gvcidr::W`](W) writer structure
impl crate::Writable for GVCIDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GVCIDR to value 0
impl crate::Resettable for GVCIDRrs {}
