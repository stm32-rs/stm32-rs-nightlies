#[doc = "Register `IDMABASER` reader"]
pub type R = crate::R<IDMABASERrs>;
#[doc = "Register `IDMABASER` writer"]
pub type W = crate::W<IDMABASERrs>;
#[doc = "Field `IDMABASE` reader - Buffer memory base address bits \\[31:2\\], must be word aligned (bit \\[1:0\\]
are always 0 and read only) This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1)."]
pub type IDMABASE_R = crate::FieldReader<u32>;
#[doc = "Field `IDMABASE` writer - Buffer memory base address bits \\[31:2\\], must be word aligned (bit \\[1:0\\]
are always 0 and read only) This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1)."]
pub type IDMABASE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer memory base address bits \\[31:2\\], must be word aligned (bit \\[1:0\\]
are always 0 and read only) This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1)."]
    #[inline(always)]
    pub fn idmabase(&self) -> IDMABASE_R {
        IDMABASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer memory base address bits \\[31:2\\], must be word aligned (bit \\[1:0\\]
are always 0 and read only) This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn idmabase(&mut self) -> IDMABASE_W<IDMABASERrs> {
        IDMABASE_W::new(self, 0)
    }
}
#[doc = "SDMMC IDMA buffer base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabaser::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabaser::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDMABASERrs;
impl crate::RegisterSpec for IDMABASERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmabaser::R`](R) reader structure"]
impl crate::Readable for IDMABASERrs {}
#[doc = "`write(|w| ..)` method takes [`idmabaser::W`](W) writer structure"]
impl crate::Writable for IDMABASERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDMABASER to value 0"]
impl crate::Resettable for IDMABASERrs {
    const RESET_VALUE: u32 = 0;
}
