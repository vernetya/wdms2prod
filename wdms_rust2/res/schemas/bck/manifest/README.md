# Manifest

The [Manifest.1.0.0](Manifest.1.0.0.json) schema describes a generic loading schema. 
Its main purpose is the validation of incoming (Storage) record fragments.

The schema definition refers to generalizations of the relevant record types,
one per each group-type:
* [GenericReferenceData.1.0.0](GenericReferenceData.1.0.0.json)
* [GenericMasterData.1.0.0](GenericMasterData.1.0.0.json)
* [GenericDataset.1.0.0](GenericDataset.1.0.0.json)
* [GenericWorkProductComponent.1.0.0](GenericWorkProductComponent.1.0.0.json)
* [GenericWorkProduct.1.0.0](GenericWorkProduct.1.0.0.json)

All are **_generated_** by the OsduSchemaComposer and pick up the shared system
properties. Only [Manifest.1.0.0](Manifest.1.0.0.json) is really authored.


