#[doc = "Register `CCCSR` reader"]
pub type R = crate::R<CCCSRrs>;
#[doc = "Register `CCCSR` writer"]
pub type W = crate::W<CCCSRrs>;
#[doc = "Field `EN1` reader - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
pub type EN1_R = crate::BitReader;
#[doc = "Field `EN1` writer - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1` reader - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS1_R = crate::BitReader;
#[doc = "Field `CS1` writer - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN2` reader - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
pub type EN2_R = crate::BitReader;
#[doc = "Field `EN2` writer - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
pub type EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS2` reader - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS2_R = crate::BitReader;
#[doc = "Field `CS2` writer - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDY1` reader - VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
pub type RDY1_R = crate::BitReader;
#[doc = "Field `RDY2` reader - VDDIO2 compensation cell ready flag This bit provides the status of the VDDIO2 compensation cell."]
pub type RDY2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDIO2 compensation cell ready flag This bit provides the status of the VDDIO2 compensation cell."]
    #[inline(always)]
    pub fn rdy2(&self) -> RDY2_R {
        RDY2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<CCCSRrs> {
        EN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<CCCSRrs> {
        CS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<CCCSRrs> {
        EN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<CCCSRrs> {
        CS2_W::new(self, 3)
    }
}
#[doc = "SBS compensation cell for I/Os control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCSRrs;
impl crate::RegisterSpec for CCCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccsr::R`](R) reader structure"]
impl crate::Readable for CCCSRrs {}
#[doc = "`write(|w| ..)` method takes [`cccsr::W`](W) writer structure"]
impl crate::Writable for CCCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCSR to value 0"]
impl crate::Resettable for CCCSRrs {
    const RESET_VALUE: u32 = 0;
}
